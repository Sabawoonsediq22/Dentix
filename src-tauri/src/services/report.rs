use sqlx::SqlitePool;
use crate::models::*;
use crate::services::errors::AppResult;
use chrono::{Utc, Datelike, NaiveDate, Duration};

fn days_in_month(year: i32, month: u32) -> i64 {
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
        .day() as i64
}

fn fill_range_daily_trends(
    start: NaiveDate,
    end: NaiveDate,
    rows: Vec<(String, f64)>,
) -> Vec<DailyTrendPoint> {
    let mut map = std::collections::HashMap::<String, f64>::new();
    for (day_str, val) in rows {
        map.insert(day_str, val);
    }
    let mut result = Vec::new();
    let mut current = start;
    while current <= end {
        let day_str = current.format("%Y-%m-%d").to_string();
        let value = map.get(&day_str).copied().unwrap_or(0.0);
        result.push(DailyTrendPoint { day: current.format("%m-%d").to_string(), value });
        current = current + Duration::days(1);
    }
    result
}

pub struct ReportService;

impl ReportService {
    pub async fn summary(pool: &SqlitePool, filter: &ReportFilter) -> AppResult<ReportSummary> {
        let now = Utc::now();

        let (start_date, end_date, prev_start, prev_end) = match filter.filter_type.as_str() {
            "daily" => {
                let today = now.format("%Y-%m-%d").to_string();
                let yesterday = (now - Duration::days(1)).format("%Y-%m-%d").to_string();
                (today.clone(), today, yesterday.clone(), yesterday)
            }
            "weekly" => {
                let week_start = (now - Duration::days(6)).format("%Y-%m-%d").to_string();
                let today = now.format("%Y-%m-%d").to_string();
                let prev_week_start = (now - Duration::days(13)).format("%Y-%m-%d").to_string();
                let prev_week_end = (now - Duration::days(7)).format("%Y-%m-%d").to_string();
                (week_start, today, prev_week_start, prev_week_end)
            }
            "custom" => {
                let start = filter.start_date.clone().unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
                let end = filter.end_date.clone().unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
                let start_dt = NaiveDate::parse_from_str(&start, "%Y-%m-%d").unwrap_or(now.date_naive());
                let end_dt = NaiveDate::parse_from_str(&end, "%Y-%m-%d").unwrap_or(now.date_naive());
                let duration = end_dt - start_dt;
                let prev_end = start_dt - Duration::days(1);
                let prev_start = prev_end - duration;
                (start, end, prev_start.format("%Y-%m-%d").to_string(), prev_end.format("%Y-%m-%d").to_string())
            }
            _ => {
                // monthly (default)
                let month_start = now.format("%Y-%m-01").to_string();
                let today = now.format("%Y-%m-%d").to_string();
                let (prev_year, prev_month) = if now.month() == 1 { (now.year() - 1, 12u32) } else { (now.year(), now.month() - 1) };
                let prev_month_start = format!("{}-{:02}-01", prev_year, prev_month);
                let prev_month_end = format!("{}-{:02}-{}", prev_year, prev_month, days_in_month(prev_year, prev_month));
                (month_start, today, prev_month_start, prev_month_end)
            }
        };

        // Parse start/end dates for daily trend calculations
        let start_dt = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").unwrap_or(now.date_naive());
        let end_dt = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap_or(now.date_naive());

        let active_patients: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT patient_id) FROM visits
             WHERE date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let total_visits: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM visits WHERE date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let completed_visits: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM visits WHERE status = 'Completed' AND date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let cancelled_visits: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM visits WHERE status = 'Cancelled' AND date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let revenue_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(COALESCE(paid_afn, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(paid_usd, 0.0)), 0.0)
             FROM invoices WHERE date(issued_at) >= ? AND date(issued_at) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let outstanding_balance_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(COALESCE(outstanding_afn, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(outstanding_usd, 0.0)), 0.0)
             FROM invoices WHERE status IN ('Unpaid', 'Partial') AND date(issued_at) >= ? AND date(issued_at) <= ?"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        // Daily trends using range-based queries
        let active_patients_rows: Vec<(String, f64)> = sqlx::query_as(
            "SELECT date(visit_date) as day_str, CAST(COUNT(DISTINCT patient_id) AS REAL) as val
             FROM visits
             WHERE date(visit_date) >= ? AND date(visit_date) <= ?
             GROUP BY date(visit_date)
             ORDER BY day_str"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;
        let active_patients_trend = fill_range_daily_trends(start_dt, end_dt, active_patients_rows);

        let visits_rows: Vec<(String, f64)> = sqlx::query_as(
            "SELECT date(visit_date) as day_str, CAST(COUNT(*) AS REAL) as val
             FROM visits
             WHERE date(visit_date) >= ? AND date(visit_date) <= ?
             GROUP BY date(visit_date)
             ORDER BY day_str"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;
        let visits_trend = fill_range_daily_trends(start_dt, end_dt, visits_rows);

        let revenue_rows: Vec<(String, f64)> = sqlx::query_as(
            "SELECT date(received_at) as day_str, COALESCE(SUM(COALESCE(amount_afn, 0.0) + COALESCE(amount_usd, 0.0)), 0.0) as val
             FROM payments
             WHERE date(received_at) >= ? AND date(received_at) <= ?
             GROUP BY date(received_at)
             ORDER BY day_str"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;
        let revenue_trend = fill_range_daily_trends(start_dt, end_dt, revenue_rows);

        let outstanding_rows: Vec<(String, f64)> = sqlx::query_as(
            "SELECT date(issued_at) as day_str, COALESCE(SUM(COALESCE(outstanding_afn, 0.0) + COALESCE(outstanding_usd, 0.0)), 0.0) as val
             FROM invoices
             WHERE status IN ('Unpaid', 'Partial') AND date(issued_at) >= ? AND date(issued_at) <= ?
             GROUP BY date(issued_at)
             ORDER BY day_str"
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;
        let outstanding_trend = fill_range_daily_trends(start_dt, end_dt, outstanding_rows);

        // Previous period comparisons
        let prev_active_patients: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT patient_id) FROM visits
             WHERE date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        let prev_total_visits: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM visits
             WHERE date(visit_date) >= ? AND date(visit_date) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        let prev_revenue: f64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(COALESCE(amount_afn, 0.0) + COALESCE(amount_usd, 0.0)), 0.0) FROM payments
             WHERE date(received_at) >= ? AND date(received_at) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        let prev_revenue_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(COALESCE(amount_afn, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(amount_usd, 0.0)), 0.0)
             FROM payments
             WHERE date(received_at) >= ? AND date(received_at) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        let prev_outstanding: f64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(COALESCE(outstanding_afn, 0.0) + COALESCE(outstanding_usd, 0.0)), 0.0) FROM invoices
             WHERE status IN ('Unpaid', 'Partial') AND date(issued_at) >= ? AND date(issued_at) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        let prev_outstanding_row: (f64, f64) = sqlx::query_as(
            "SELECT
               COALESCE(SUM(COALESCE(outstanding_afn, 0.0)), 0.0),
               COALESCE(SUM(COALESCE(outstanding_usd, 0.0)), 0.0)
             FROM invoices
             WHERE status IN ('Unpaid', 'Partial') AND date(issued_at) >= ? AND date(issued_at) <= ?"
        )
        .bind(&prev_start)
        .bind(&prev_end)
        .fetch_one(pool)
        .await?;

        Ok(ReportSummary {
            active_patients,
            total_visits_this_month: total_visits,
            revenue_this_month: revenue_row.0 + revenue_row.1,
            revenue_this_month_afn: revenue_row.0,
            revenue_this_month_usd: revenue_row.1,
            outstanding_balance: outstanding_balance_row.0 + outstanding_balance_row.1,
            outstanding_balance_afn: outstanding_balance_row.0,
            outstanding_balance_usd: outstanding_balance_row.1,
            completed_visits_this_month: completed_visits,
            cancelled_visits_this_month: cancelled_visits,
            active_patients_trend,
            visits_trend,
            revenue_trend,
            outstanding_trend,
            prev_active_patients,
            prev_total_visits,
            prev_revenue,
            prev_revenue_afn: prev_revenue_row.0,
            prev_revenue_usd: prev_revenue_row.1,
            prev_outstanding,
            prev_outstanding_afn: prev_outstanding_row.0,
            prev_outstanding_usd: prev_outstanding_row.1,
        })
    }

    pub async fn monthly_revenue(pool: &SqlitePool, filter: &ReportFilter) -> AppResult<Vec<MonthlyRevenuePoint>> {
        let now = Utc::now();

        let (start_date, end_date) = match filter.filter_type.as_str() {
            "daily" => {
                let today = now.format("%Y-%m-%d").to_string();
                (today.clone(), today)
            }
            "weekly" => {
                let week_start = (now - Duration::days(6)).format("%Y-%m-%d").to_string();
                let today = now.format("%Y-%m-%d").to_string();
                (week_start, today)
            }
            "custom" => {
                let start = filter.start_date.clone().unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
                let end = filter.end_date.clone().unwrap_or_else(|| now.format("%Y-%m-%d").to_string());
                (start, end)
            }
            _ => {
                // monthly (default) - show last 12 months
                let twelve_months_ago = (now - Duration::days(365)).format("%Y-%m-%d").to_string();
                let today = now.format("%Y-%m-%d").to_string();
                (twelve_months_ago, today)
            }
        };

        let rows: Vec<(String, f64, f64, f64)> = if filter.filter_type == "daily" || filter.filter_type == "weekly" || filter.filter_type == "custom" {
            sqlx::query_as(
                "SELECT date(issued_at) as month,
                        COALESCE(SUM(COALESCE(paid_afn, 0.0) + COALESCE(paid_usd, 0.0)), 0.0) as revenue,
                        COALESCE(SUM(COALESCE(paid_afn, 0.0)), 0.0) as revenue_afn,
                        COALESCE(SUM(COALESCE(paid_usd, 0.0)), 0.0) as revenue_usd
                 FROM invoices
                 WHERE date(issued_at) >= ? AND date(issued_at) <= ?
                 GROUP BY date(issued_at)
                 ORDER BY month ASC"
            )
            .bind(&start_date)
            .bind(&end_date)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT strftime('%Y-%m', issued_at) as month,
                        COALESCE(SUM(COALESCE(paid_afn, 0.0) + COALESCE(paid_usd, 0.0)), 0.0) as revenue,
                        COALESCE(SUM(COALESCE(paid_afn, 0.0)), 0.0) as revenue_afn,
                        COALESCE(SUM(COALESCE(paid_usd, 0.0)), 0.0) as revenue_usd
                 FROM invoices
                 WHERE date(issued_at) >= ? AND date(issued_at) <= ?
                 GROUP BY strftime('%Y-%m', issued_at)
                 ORDER BY month ASC"
            )
            .bind(&start_date)
            .bind(&end_date)
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|(month, revenue, revenue_afn, revenue_usd)| MonthlyRevenuePoint { month, revenue, revenue_afn, revenue_usd }).collect())
    }
}
