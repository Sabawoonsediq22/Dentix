use crate::models::*;
use crate::services::errors::{AppError, AppResult};
use chrono::Utc;
use sqlx::SqlitePool;

pub struct InvoiceService;

#[allow(dead_code)]
fn calculate_invoice_status(outstanding_amount: f64, paid_amount: f64) -> InvoiceStatus {
    if outstanding_amount == 0.0 {
        InvoiceStatus::Paid
    } else if paid_amount > 0.0 {
        InvoiceStatus::Partial
    } else {
        InvoiceStatus::Unpaid
    }
}

impl InvoiceService {
    pub async fn list(pool: &SqlitePool, params: crate::models::InvoiceListParams) -> AppResult<InvoicePageResult> {
        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(10).max(1);
        let offset = ((page - 1) * per_page) as i64;
        let per_page_i64 = per_page as i64;

        let mut conditions: Vec<String> = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        if let Some(ref q) = params.query {
            if !q.trim().is_empty() {
                conditions.push("(i.invoice_number LIKE ? OR p.full_name LIKE ? OR p.phone LIKE ? OR i.id = ?)".to_string());
                let like = format!("%{}%", q.trim());
                bind_values.push(like.clone());
                bind_values.push(like.clone());
                bind_values.push(like.clone());
                bind_values.push(q.trim().to_string());
            }
        }

        if let Some(ref pid) = params.patient_id {
            if !pid.trim().is_empty() {
                conditions.push("v.patient_id = ?".to_string());
                bind_values.push(pid.trim().to_string());
            }
        }

        if let Some(ref s) = params.status {
            if s != "All" {
                conditions.push("i.status = ?".to_string());
                bind_values.push(s.clone());
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        // Build total query
        let total_query_str = if bind_values.is_empty() {
            "SELECT COUNT(*) FROM invoices i".to_string()
        } else {
            format!("SELECT COUNT(*) FROM invoices i JOIN visits v ON v.id = i.visit_id JOIN patients p ON p.id = v.patient_id {}", where_clause)
        };
        
        let mut total_query = sqlx::query_scalar(&total_query_str);
        for val in &bind_values {
            total_query = total_query.bind(val);
        }
        let total = total_query.fetch_one(pool).await?;

        // Prepare status count query values (respecting query and patient_id filters but ignoring status filter)
        let mut status_bind_values: Vec<String> = Vec::new();
        let mut status_extra_conditions: Vec<String> = Vec::new();
        if let Some(ref q) = params.query {
            if !q.trim().is_empty() {
                let like = format!("%{}%", q.trim());
                status_bind_values.push(like.clone());
                status_bind_values.push(like.clone());
                status_bind_values.push(like.clone());
                status_bind_values.push(q.trim().to_string());
            }
        }
        if let Some(ref pid) = params.patient_id {
            if !pid.trim().is_empty() {
                status_extra_conditions.push("v.patient_id = ?".to_string());
                status_bind_values.push(pid.trim().to_string());
            }
        }

        // Build a status-where clause (respects query + patient_id, ignores status)
        let has_query_filter = params.query.as_ref().map_or(false, |q| !q.trim().is_empty());
        let has_patient_filter = params.patient_id.as_ref().map_or(false, |p| !p.trim().is_empty());
        let needs_join = has_query_filter || has_patient_filter;

        let mut status_where_parts: Vec<String> = Vec::new();
        if has_query_filter {
            status_where_parts.push("(i.invoice_number LIKE ? OR p.full_name LIKE ? OR p.phone LIKE ? OR i.id = ?)".to_string());
        }
        for extra in &status_extra_conditions {
            status_where_parts.push(extra.clone());
        }
        let status_where_clause = if status_where_parts.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", status_where_parts.join(" AND "))
        };
        let status_from_clause = if needs_join {
            "FROM invoices i JOIN visits v ON v.id = i.visit_id JOIN patients p ON p.id = v.patient_id"
        } else {
            "FROM invoices i"
        };

        // Query status counts
        let unpaid_count_sql = if status_where_clause.is_empty() {
            format!("SELECT COUNT(*) {} WHERE i.status = 'Unpaid'", status_from_clause)
        } else {
            format!("SELECT COUNT(*) {}{} AND i.status = 'Unpaid'", status_from_clause, status_where_clause)
        };
        let unpaid_count: i64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&unpaid_count_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&unpaid_count_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        let partial_count_sql = if status_where_clause.is_empty() {
            format!("SELECT COUNT(*) {} WHERE i.status = 'Partial'", status_from_clause)
        } else {
            format!("SELECT COUNT(*) {}{} AND i.status = 'Partial'", status_from_clause, status_where_clause)
        };
        let partial_count: i64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&partial_count_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&partial_count_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        let paid_count_sql = if status_where_clause.is_empty() {
            format!("SELECT COUNT(*) {} WHERE i.status = 'Paid'", status_from_clause)
        } else {
            format!("SELECT COUNT(*) {}{} AND i.status = 'Paid'", status_from_clause, status_where_clause)
        };
        let paid_count: i64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&paid_count_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&paid_count_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        let total_outstanding_sql = if status_where_clause.is_empty() {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_afn, 0.0) + COALESCE(i.outstanding_usd, 0.0)), 0.0) {} WHERE i.status IN ('Unpaid', 'Partial')",
                status_from_clause
            )
        } else {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_afn, 0.0) + COALESCE(i.outstanding_usd, 0.0)), 0.0) {}{} AND i.status IN ('Unpaid', 'Partial')",
                status_from_clause, status_where_clause
            )
        };
        let total_outstanding: f64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&total_outstanding_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&total_outstanding_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        // Per-currency total outstanding
        let total_outstanding_afn_sql = if status_where_clause.is_empty() {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_afn, 0.0)), 0.0) {} WHERE i.status IN ('Unpaid', 'Partial')",
                status_from_clause
            )
        } else {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_afn, 0.0)), 0.0) {}{} AND i.status IN ('Unpaid', 'Partial')",
                status_from_clause, status_where_clause
            )
        };
        let _total_outstanding_afn: f64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&total_outstanding_afn_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&total_outstanding_afn_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        let total_outstanding_usd_sql = if status_where_clause.is_empty() {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_usd, 0.0)), 0.0) {} WHERE i.status IN ('Unpaid', 'Partial')",
                status_from_clause
            )
        } else {
            format!(
                "SELECT COALESCE(SUM(COALESCE(i.outstanding_usd, 0.0)), 0.0) {}{} AND i.status IN ('Unpaid', 'Partial')",
                status_from_clause, status_where_clause
            )
        };
        let _total_outstanding_usd: f64 = if status_bind_values.is_empty() {
            sqlx::query_scalar(&total_outstanding_usd_sql).fetch_one(pool).await?
        } else {
            let mut q = sqlx::query_scalar(&total_outstanding_usd_sql);
            for val in &status_bind_values {
                q = q.bind(val);
            }
            q.fetch_one(pool).await?
        };

        // Build main query
        let query_str = format!(
            "SELECT i.id, i.invoice_number,
                    COALESCE(i.subtotal_afn, 0.0) as subtotal_afn,
                    COALESCE(i.subtotal_usd, 0.0) as subtotal_usd,
                    COALESCE(i.discount_afn, 0.0) as discount_afn,
                    COALESCE(i.discount_usd, 0.0) as discount_usd,
                    COALESCE(i.total_afn, 0.0) as total_afn,
                    COALESCE(i.total_usd, 0.0) as total_usd,
                    COALESCE(i.paid_afn, 0.0) as paid_afn,
                    COALESCE(i.paid_usd, 0.0) as paid_usd,
                    COALESCE(i.outstanding_afn, 0.0) as outstanding_afn,
                    COALESCE(i.outstanding_usd, 0.0) as outstanding_usd,
                    i.status, i.issued_at, i.visit_id as visit_id, v.patient_id as patient_id, p.full_name as patient_name, p.phone as patient_phone, v.visit_date
             FROM invoices i
             JOIN visits v ON v.id = i.visit_id
             JOIN patients p ON p.id = v.patient_id {} ORDER BY i.issued_at DESC LIMIT ? OFFSET ?",
            where_clause
        );

        let mut query = sqlx::query_as::<_, InvoiceListItem>(&query_str);

        for val in bind_values {
            query = query.bind(val);
        }
        query = query.bind(per_page_i64).bind(offset);

        let items: Vec<InvoiceListItem> = query.fetch_all(pool).await?;

        Ok(InvoicePageResult {
            items,
            total,
            page,
            per_page,
            total_pages: ((total + per_page_i64 - 1) / per_page_i64).max(1),
            unpaid_count,
            partial_count,
            paid_count,
            total_outstanding,
        })
    }

    pub async fn create(pool: &SqlitePool, input: CreateInvoiceInput) -> AppResult<Invoice> {
        let id = format!("INV-{}", uuid::Uuid::new_v4().simple());
        let invoice_number = format!("INV-{}", Utc::now().timestamp_millis());

        let now = Utc::now().to_rfc3339();

        let row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(p.procedure_price_afn * tr.number_of_procedures), 0.0),
               COALESCE(SUM(p.procedure_price_usd * tr.number_of_procedures), 0.0)
             FROM treatment_records tr
             JOIN procedures p ON p.id = tr.procedure_id
             WHERE tr.visit_id = ?",
        )
        .bind(&input.visit_id)
        .fetch_one(pool)
        .await?;

        let subtotal_afn = row.0;
        let subtotal_usd = row.1;

        let total_afn = (subtotal_afn - input.discount_afn).max(0.0);
        let total_usd = (subtotal_usd - input.discount_usd).max(0.0);
        let paid_afn = input.paid_amount_afn;
        let paid_usd = input.paid_amount_usd;
        let final_paid_afn = if total_afn - paid_afn < 0.0 { total_afn } else { paid_afn.min(total_afn) };
        let final_paid_usd = if total_usd - paid_usd < 0.0 { total_usd } else { paid_usd.min(total_usd) };
        let final_outstanding_afn = (total_afn - final_paid_afn).max(0.0);
        let final_outstanding_usd = (total_usd - final_paid_usd).max(0.0);

        let status = if final_outstanding_afn == 0.0 && final_outstanding_usd == 0.0 {
            InvoiceStatus::Paid
        } else if final_paid_afn > 0.0 || final_paid_usd > 0.0 {
            InvoiceStatus::Partial
        } else {
            InvoiceStatus::Unpaid
        };

        let invoice = sqlx::query_as::<_, Invoice>(
            "INSERT INTO invoices (id, visit_id, invoice_number,
              subtotal_afn, subtotal_usd,
              discount_afn, discount_usd,
              total_afn, total_usd,
              paid_afn, paid_usd,
              outstanding_afn, outstanding_usd,
              status, issued_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             RETURNING id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at"
        )
        .bind(&id)
        .bind(&input.visit_id)
        .bind(&invoice_number)
        .bind(subtotal_afn)
        .bind(subtotal_usd)
        .bind(input.discount_afn)
        .bind(input.discount_usd)
        .bind(total_afn)
        .bind(total_usd)
        .bind(final_paid_afn)
        .bind(final_paid_usd)
        .bind(final_outstanding_afn)
        .bind(final_outstanding_usd)
        .bind(match status {
            InvoiceStatus::Unpaid => "Unpaid",
            InvoiceStatus::Partial => "Partial",
            InvoiceStatus::Paid => "Paid",
        })
        .bind(&now)
        .fetch_one(pool)
        .await?;

        if final_paid_afn > 0.0 || final_paid_usd > 0.0 {
            sqlx::query(
                "INSERT INTO payments (id, invoice_id, amount_afn, amount_usd, method, notes, received_at)
                 VALUES (?, ?, ?, ?, 'Cash', '', ?)",
            )
            .bind(format!("PAY-{}", uuid::Uuid::new_v4().simple()))
            .bind(&id)
            .bind(final_paid_afn)
            .bind(final_paid_usd)
            .bind(&now)
            .execute(pool)
            .await?;
        }

        Ok(invoice)
    }

    pub async fn get_for_visit(pool: &SqlitePool, visit_id: &str) -> AppResult<Option<Invoice>> {
        let invoice = sqlx::query_as(
            "SELECT id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at
             FROM invoices WHERE visit_id = ?"
        )
        .bind(visit_id)
        .fetch_optional(pool)
        .await?;

        Ok(invoice)
    }

    pub async fn get_receipt_details(
        pool: &SqlitePool,
        invoice_id: &str,
    ) -> AppResult<ReceiptData> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at
             FROM invoices WHERE id = ?"
        )
        .bind(invoice_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Invoice {} not found", invoice_id)))?;

        let patient = sqlx::query_as::<_, ReceiptPatient>(
            "SELECT p.id, p.full_name, p.phone FROM patients p
             INNER JOIN visits v ON v.patient_id = p.id
             WHERE v.id = ?",
        )
        .bind(&invoice.visit_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!("Patient for visit {} not found", invoice.visit_id))
        })?;

        let payments = sqlx::query_as::<_, ReceiptPayment>(
            "SELECT id, invoice_id, amount_afn, amount_usd, method, notes, received_at FROM payments
             WHERE invoice_id = ?
             ORDER BY received_at ASC",
        )
        .bind(&invoice.id)
        .fetch_all(pool)
        .await?;

        let procedure_rows = sqlx::query_as::<_, ReceiptProcedure>(
            "SELECT tr.id as treatment_record_id,
                    p.name as procedure_name,
                    p.additional_note,
                    tr.number_of_procedures as quantity,
                    p.procedure_price_afn as unit_price_afn,
                    p.procedure_price_usd as unit_price_usd,
                    (p.procedure_price_afn * tr.number_of_procedures) as total_price_afn,
                    (p.procedure_price_usd * tr.number_of_procedures) as total_price_usd,
                    tr.performed_at,
                    (SELECT GROUP_CONCAT(tt.tooth_number, ', ') FROM treatment_tooth tt WHERE tt.treatment_record_id = tr.id) as tooth_numbers
             FROM treatment_records tr
             INNER JOIN procedures p ON p.id = tr.procedure_id
             WHERE tr.visit_id = ?
             ORDER BY tr.performed_at DESC"
        )
        .bind(&invoice.visit_id)
        .fetch_all(pool)
        .await?;

        let clinic_settings = sqlx::query_as::<_, (Option<String>, Option<String>, Option<String>, Option<String>)>(
            "SELECT clinic_name, clinic_address, clinic_phone, clinic_logo FROM app_settings WHERE id = 1"
        )
        .fetch_optional(pool)
        .await?;

        let (clinic_name, clinic_address, clinic_phone, clinic_logo) = clinic_settings.unwrap_or_default();

        Ok(ReceiptData {
            id: invoice.id,
            invoice_number: invoice.invoice_number,
            patient_id: patient.id,
            patient_name: patient.full_name,
            patient_phone: patient.phone,
            visit_id: invoice.visit_id,
            issue_date: invoice.issued_at,
            currency: "AFN".to_string(),
            subtotal_afn: invoice.subtotal_afn,
            subtotal_usd: invoice.subtotal_usd,
            discount_afn: invoice.discount_afn,
            discount_usd: invoice.discount_usd,
            total_afn: invoice.total_afn,
            total_usd: invoice.total_usd,
            paid_afn: invoice.paid_afn,
            paid_usd: invoice.paid_usd,
            outstanding_afn: invoice.outstanding_afn,
            outstanding_usd: invoice.outstanding_usd,
            status: invoice.status,
            procedures: procedure_rows,
            payments,
            clinic: ReceiptClinic {
                name: clinic_name.unwrap_or_else(|| "Dental Clinic".to_string()),
                address: clinic_address.unwrap_or_default(),
                phone: clinic_phone.unwrap_or_default(),
                logo_url: clinic_logo,
            },
        })
    }

    pub async fn get_receipt_details_by_visit(
        pool: &SqlitePool,
        visit_id: &str,
    ) -> AppResult<ReceiptData> {
        let invoice = sqlx::query_as::<_, Invoice>(
            "SELECT id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at
             FROM invoices WHERE visit_id = ?"
        )
        .bind(visit_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Invoice for visit {} not found", visit_id)))?;

        Self::get_receipt_details(pool, &invoice.id).await
    }

    #[allow(dead_code)]
    pub async fn find(pool: &SqlitePool, id: &str) -> AppResult<Invoice> {
        let invoice = sqlx::query_as(
            "SELECT id, visit_id, invoice_number,
               subtotal_afn, subtotal_usd,
               discount_afn, discount_usd,
               total_afn, total_usd,
               paid_afn, paid_usd,
               outstanding_afn, outstanding_usd,
               status, issued_at
             FROM invoices WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Invoice {} not found", id)))?;

        Ok(invoice)
    }
}