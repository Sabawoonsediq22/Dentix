use crate::models::*;
use crate::services::errors::{AppError, AppResult};
use chrono::Utc;
use sqlx::SqlitePool;

pub struct ExpenseService;

fn payment_method_str(method: PaymentMethod) -> &'static str {
    match method {
        PaymentMethod::Cash => "Cash",
        PaymentMethod::Card => "Card",
        PaymentMethod::Mobile => "Mobile",
        PaymentMethod::Insurance => "Insurance",
    }
}

fn validate_amounts(amount_afn: f64, amount_usd: f64) -> AppResult<()> {
    if amount_afn < 0.0 || amount_usd < 0.0 {
        return Err(AppError::InvalidInput("amount cannot be negative".to_string()));
    }
    if amount_afn == 0.0 && amount_usd == 0.0 {
        return Err(AppError::InvalidInput("at least one amount is required".to_string()));
    }
    Ok(())
}

async fn category_exists(pool: &SqlitePool, category_id: &str) -> AppResult<()> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM expense_categories WHERE id = ?)")
        .bind(category_id)
        .fetch_one(pool)
        .await?;
    if !exists {
        return Err(AppError::InvalidInput("expense category not found".to_string()));
    }
    Ok(())
}

impl ExpenseService {
    pub async fn list(pool: &SqlitePool, params: ExpenseListParams) -> AppResult<ExpensePageResult> {
        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(10).max(1);
        let offset = ((page - 1) * per_page) as i64;
        let per_page_i64 = per_page as i64;

        let mut conditions: Vec<String> = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        if let Some(ref q) = params.query {
            if !q.trim().is_empty() {
                conditions.push("(e.description LIKE ? OR e.notes LIKE ? OR c.name LIKE ? OR e.id = ?)".to_string());
                let like = format!("%{}%", q.trim());
                bind_values.push(like.clone());
                bind_values.push(like.clone());
                bind_values.push(like.clone());
                bind_values.push(q.trim().to_string());
            }
        }

        if let Some(ref category_id) = params.category_id {
            if !category_id.trim().is_empty() {
                conditions.push("e.category_id = ?".to_string());
                bind_values.push(category_id.trim().to_string());
            }
        }

        if let Some(ref start) = params.start_date {
            if !start.trim().is_empty() {
                conditions.push("date(e.expense_date) >= date(?)".to_string());
                bind_values.push(start.trim().to_string());
            }
        }

        if let Some(ref end) = params.end_date {
            if !end.trim().is_empty() {
                conditions.push("date(e.expense_date) <= date(?)".to_string());
                bind_values.push(end.trim().to_string());
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };
        let from_clause = "FROM expenses e JOIN expense_categories c ON c.id = e.category_id";

        // Total count (filtered)
        let count_sql = format!("SELECT COUNT(*) {}{}", from_clause, where_clause);
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
        for val in &bind_values {
            count_query = count_query.bind(val);
        }
        let total = count_query.fetch_one(pool).await?;

        // Filtered totals per currency
        let sum_sql = format!(
            "SELECT COALESCE(SUM(COALESCE(e.amount_afn, 0.0)), 0.0), COALESCE(SUM(COALESCE(e.amount_usd, 0.0)), 0.0) {}{}",
            from_clause, where_clause
        );
        let mut sum_query = sqlx::query_as::<_, (f64, f64)>(&sum_sql);
        for val in &bind_values {
            sum_query = sum_query.bind(val);
        }
        let (total_spent_afn, total_spent_usd) = sum_query.fetch_one(pool).await?;

        // Page items
        let query_str = format!(
            "SELECT e.id, e.category_id, c.name as category_name, c.name_ps as category_name_ps,
                    e.description,
                    COALESCE(e.amount_afn, 0.0) as amount_afn,
                    COALESCE(e.amount_usd, 0.0) as amount_usd,
                    e.expense_date, e.payment_method, e.notes, e.created_at, e.updated_at
             {}{} ORDER BY date(e.expense_date) DESC, e.created_at DESC LIMIT ? OFFSET ?",
            from_clause, where_clause
        );
        let mut query = sqlx::query_as::<_, ExpenseListItem>(&query_str);
        for val in &bind_values {
            query = query.bind(val);
        }
        query = query.bind(per_page_i64).bind(offset);
        let items: Vec<ExpenseListItem> = query.fetch_all(pool).await?;

        Ok(ExpensePageResult {
            items,
            total,
            page,
            per_page,
            total_pages: ((total + per_page_i64 - 1) / per_page_i64).max(1),
            total_spent_afn,
            total_spent_usd,
        })
    }

    pub async fn find(pool: &SqlitePool, id: &str) -> AppResult<Expense> {
        let expense = sqlx::query_as::<_, Expense>(
            "SELECT id, category_id, description, amount_afn, amount_usd, expense_date, payment_method, notes, created_at, updated_at
             FROM expenses WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Expense {} not found", id)))?;

        Ok(expense)
    }

    pub async fn create(pool: &SqlitePool, input: CreateExpenseInput) -> AppResult<Expense> {
        if input.expense_date.trim().is_empty() {
            return Err(AppError::InvalidInput("expense date is required".to_string()));
        }
        validate_amounts(input.amount_afn, input.amount_usd)?;
        category_exists(pool, &input.category_id).await?;

        let id = format!("EXP-{}", uuid::Uuid::new_v4().simple());
        let now = Utc::now().to_rfc3339();
        let method_str = payment_method_str(input.payment_method);

        let expense = sqlx::query_as::<_, Expense>(
            "INSERT INTO expenses (id, category_id, description, amount_afn, amount_usd, expense_date, payment_method, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id, category_id, description, amount_afn, amount_usd, expense_date, payment_method, notes, created_at, updated_at"
        )
        .bind(&id)
        .bind(&input.category_id)
        .bind(input.description.trim())
        .bind(input.amount_afn)
        .bind(input.amount_usd)
        .bind(input.expense_date.trim())
        .bind(method_str)
        .bind(input.notes.as_deref().unwrap_or("").trim())
        .bind(&now)
        .bind(&now)
        .fetch_one(pool)
        .await?;

        Ok(expense)
    }

    pub async fn update(pool: &SqlitePool, id: &str, input: UpdateExpenseInput) -> AppResult<Expense> {
        if input.expense_date.trim().is_empty() {
            return Err(AppError::InvalidInput("expense date is required".to_string()));
        }
        validate_amounts(input.amount_afn, input.amount_usd)?;
        category_exists(pool, &input.category_id).await?;

        let now = Utc::now().to_rfc3339();
        let method_str = payment_method_str(input.payment_method);

        let expense = sqlx::query_as::<_, Expense>(
            "UPDATE expenses SET
             category_id = ?, description = ?, amount_afn = ?, amount_usd = ?,
             expense_date = ?, payment_method = ?, notes = ?, updated_at = ?
             WHERE id = ?
             RETURNING id, category_id, description, amount_afn, amount_usd, expense_date, payment_method, notes, created_at, updated_at"
        )
        .bind(&input.category_id)
        .bind(input.description.trim())
        .bind(input.amount_afn)
        .bind(input.amount_usd)
        .bind(input.expense_date.trim())
        .bind(method_str)
        .bind(input.notes.as_deref().unwrap_or("").trim())
        .bind(&now)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Expense {} not found", id)))?;

        Ok(expense)
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM expenses WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Expense {} not found", id)));
        }

        Ok(())
    }

    pub async fn list_categories(pool: &SqlitePool) -> AppResult<Vec<ExpenseCategory>> {
        let categories = sqlx::query_as::<_, ExpenseCategory>(
            "SELECT id, name, name_ps, sort_order FROM expense_categories ORDER BY sort_order ASC, name ASC"
        )
        .fetch_all(pool)
        .await?;

        Ok(categories)
    }
}
