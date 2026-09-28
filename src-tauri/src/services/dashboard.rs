use sqlx::SqlitePool;
use crate::models::*;
use crate::services::errors::AppResult;
use chrono::{Utc, Duration, Local};

pub struct DashboardService;

impl DashboardService {
    pub async fn stats(pool: &SqlitePool) -> AppResult<DashboardStats> {
        let now = Local::now();
        let today = now.format("%Y-%m-%d").to_string();
        let yesterday = (now - Duration::days(1)).format("%Y-%m-%d").to_string();

        let daily_revenue_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(amount_afn), 0.0),
               COALESCE(SUM(amount_usd), 0.0)
             FROM payments WHERE date(received_at, 'localtime') = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await?;

        let patients_today: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT patient_id) FROM visits
             WHERE date(CASE WHEN length(visit_date) > 10 THEN date(visit_date, 'localtime') ELSE visit_date END) = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await?;

        let outstanding_row: (f64, f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(COALESCE(outstanding_afn, 0.0) + COALESCE(outstanding_usd, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(outstanding_afn, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(outstanding_usd, 0.0)), 0.0)
             FROM invoices WHERE status IN ('Unpaid', 'Partial')"
        )
        .fetch_one(pool)
        .await?;

        let outstanding_invoices_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM invoices WHERE status IN ('Unpaid', 'Partial')"
        )
        .fetch_one(pool)
        .await?;

        let procedures_performed: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(number_of_procedures), 0) FROM treatment_records WHERE date(performed_at, 'localtime') = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await?;

        let yesterday_revenue_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(amount_afn), 0.0),
               COALESCE(SUM(amount_usd), 0.0)
             FROM payments WHERE date(received_at, 'localtime') = ?"
        )
        .bind(&yesterday)
        .fetch_one(pool)
        .await?;

        let yesterday_patients: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT patient_id) FROM visits
             WHERE date(CASE WHEN length(visit_date) > 10 THEN date(visit_date, 'localtime') ELSE visit_date END) = ?"
        )
        .bind(&yesterday)
        .fetch_one(pool)
        .await?;

        let yesterday_procedures: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(number_of_procedures), 0) FROM treatment_records WHERE date(performed_at, 'localtime') = ?"
        )
        .bind(&yesterday)
        .fetch_one(pool)
        .await?;

        let daily_expenses_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(amount_afn), 0.0),
               COALESCE(SUM(amount_usd), 0.0)
             FROM expenses WHERE date(expense_date) = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await?;

        let yesterday_expenses_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(amount_afn), 0.0),
               COALESCE(SUM(amount_usd), 0.0)
             FROM expenses WHERE date(expense_date) = ?"
        )
        .bind(&yesterday)
        .fetch_one(pool)
        .await?;

        let net_today_afn = daily_revenue_row.0 - daily_expenses_row.0;
        let net_today_usd = daily_revenue_row.1 - daily_expenses_row.1;

        Ok(DashboardStats {
            daily_revenue: daily_revenue_row.0 + daily_revenue_row.1,
            daily_revenue_afn: daily_revenue_row.0,
            daily_revenue_usd: daily_revenue_row.1,
            patients_today,
            outstanding_balance: outstanding_row.0,
            outstanding_balance_afn: outstanding_row.1,
            outstanding_balance_usd: outstanding_row.2,
            outstanding_invoices_count,
            procedures_performed,
            yesterday_revenue: yesterday_revenue_row.0 + yesterday_revenue_row.1,
            yesterday_revenue_afn: yesterday_revenue_row.0,
            yesterday_revenue_usd: yesterday_revenue_row.1,
            yesterday_patients,
            yesterday_procedures,
            daily_expenses: daily_expenses_row.0 + daily_expenses_row.1,
            daily_expenses_afn: daily_expenses_row.0,
            daily_expenses_usd: daily_expenses_row.1,
            yesterday_expenses: yesterday_expenses_row.0 + yesterday_expenses_row.1,
            yesterday_expenses_afn: yesterday_expenses_row.0,
            yesterday_expenses_usd: yesterday_expenses_row.1,
            net_today: net_today_afn + net_today_usd,
            net_today_afn,
            net_today_usd,
        })
    }

    pub async fn patients_flow(pool: &SqlitePool, mode: &str) -> AppResult<Vec<PatientsFlowPoint>> {
        if mode == "weekly" {
            let mut result = Vec::new();
            for i in (0..7).rev() {
                let day = Utc::now() - Duration::days(i);
                let date_str = day.format("%Y-%m-%d").to_string();
                let label = day.format("%a").to_string();

                let check_ins: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM visits WHERE date(visit_date) = ?"
                )
                .bind(&date_str)
                .fetch_one(pool)
                .await?;

                let completed: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM visits WHERE date(visit_date) = ? AND status = 'Completed'"
                )
                .bind(&date_str)
                .fetch_one(pool)
                .await?;

                result.push(PatientsFlowPoint {
                    label,
                    check_ins,
                    visits: check_ins,
                    completed,
                });
            }
            return Ok(result);
        }

        if mode == "monthly" {
            let mut result = Vec::new();
            let now = Utc::now();
            for i in (0..12).rev() {
                let month = now - Duration::days(i as i64 * 30);
                let year = month.format("%Y").to_string();
                let month_num = month.format("%m").to_string();
                let label = month.format("%b %Y").to_string();

                let check_ins: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM visits WHERE strftime('%Y', visit_date) = ? AND strftime('%m', visit_date) = ?"
                )
                .bind(&year)
                .bind(&month_num)
                .fetch_one(pool)
                .await?;

                let completed: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM visits WHERE strftime('%Y', visit_date) = ? AND strftime('%m', visit_date) = ? AND status = 'Completed'"
                )
                .bind(&year)
                .bind(&month_num)
                .fetch_one(pool)
                .await?;

                result.push(PatientsFlowPoint {
                    label,
                    check_ins,
                    visits: check_ins,
                    completed,
                });
            }
            return Ok(result);
        }

        let mut result = Vec::new();
        for hour in [8u32, 10, 12, 14, 16, 18] {
            let hour_str = format!("{:02}:00", hour);
            let next_hour = format!("{:02}:59", hour);

            let check_ins: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM visits WHERE visit_date >= time(?) AND visit_date < time(?)"
            )
            .bind(&hour_str)
            .bind(&next_hour)
            .fetch_one(pool)
            .await?;

            let completed: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM visits WHERE visit_date >= time(?) AND visit_date < time(?) AND status = 'Completed'"
            )
            .bind(&hour_str)
            .bind(&next_hour)
            .fetch_one(pool)
            .await?;

            result.push(PatientsFlowPoint {
                label: hour_str,
                check_ins,
                visits: check_ins,
                completed,
            });
        }
        Ok(result)
    }

    pub async fn procedure_distribution(pool: &SqlitePool, mode: &str) -> AppResult<Vec<ProcedureDistribution>> {
        let date_filter = match mode {
            "weekly" => "date(tr.performed_at) >= date('now', '-7 days')",
            "monthly" => "date(tr.performed_at) >= date('now', '-30 days')",
            _ => "date(tr.performed_at) = date('now')",
        };

        let query = format!(
            "SELECT p.name, COUNT(*) as count FROM treatment_records tr
             JOIN procedures p ON tr.procedure_id = p.id
             WHERE {}
             GROUP BY p.name
             ORDER BY count DESC",
            date_filter
        );

        let rows: Vec<(String, i64)> = sqlx::query_as(&query)
            .fetch_all(pool)
            .await?;

        Ok(rows.into_iter().map(|(name, count)| ProcedureDistribution { name, count }).collect())
    }

    pub async fn recent_patients(pool: &SqlitePool) -> AppResult<Vec<RecentPatient>> {
        let rows: Vec<(String, String, String, i32, String, String, String, String, String)> = sqlx::query_as(
            "SELECT p.id, p.full_name, p.phone, p.age, p.gender, p.address, v.visit_date, v.status, v.id
             FROM patients p
             JOIN visits v ON p.id = v.patient_id
             ORDER BY v.visit_date DESC
             LIMIT 10"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(id, full_name, phone, age, gender, address, visit_date, status, visit_id)| {
            RecentPatient {
                id,
                full_name,
                phone,
                age,
                gender,
                address,
                visit_date,
                status,
                visit_id,
            }
        }).collect())
    }
}
