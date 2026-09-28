use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::config::AppConfig;

// Patient
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: String,
    pub full_name: String,
    pub phone: String,
    pub age: i32,
    pub gender: Gender,
    pub address: Option<String>,
    pub last_visit: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CreatedPatient {
    pub id: String,
    pub full_name: String,
    pub phone: String,
    pub age: i32,
    pub gender: Gender,
    pub address: Option<String>,
    pub last_visit: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub visit_id: String,
    pub invoice_id: String,
    pub invoice_number: String,
    pub treatment_record_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Other,
}

// Patient medical info
#[derive(Debug, Serialize, Deserialize)]
pub struct PatientMedicalInfoResponse {
    pub allergies: Vec<String>,
    pub medical_conditions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientStatisticsResponse {
    pub total_spent: f64,
    pub total_spent_afn: f64,
    pub total_spent_usd: f64,
    pub last_visit_date: Option<String>,
    pub last_visit_procedure: Option<String>,
    pub outstanding_balance: f64,
    pub outstanding_balance_afn: f64,
    pub outstanding_balance_usd: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct MedicalCondition {
    pub id: i64,
    pub patient_id: String,
    pub condition_name: String,
    pub is_active: bool,
}

// Visit
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Visit {
    pub id: String,
    pub patient_id: String,
    pub visit_date: String,
    pub chief_complaint: Option<String>,
    pub status: VisitStatus,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
pub enum VisitStatus {
    Open,
    Completed,
    Cancelled,
}

// Treatment
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TreatmentRecord {
    pub id: String,
    pub visit_id: String,
    pub procedure_id: String,
    pub number_of_procedures: i32,
    pub performed_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[allow(dead_code)]
pub struct TreatmentTooth {
    pub id: i64,
    pub treatment_record_id: String,
    pub tooth_number: i32,
    pub tooth_quadrant: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatmentProcedure {
    pub treatment_record_id: String,
    pub procedure_name: String,
    pub procedure_additional_note: Option<String>,
    pub number_of_procedures: i32,
    pub unit_price_afn: f64,
    pub unit_price_usd: f64,
    pub total_price_afn: f64,
    pub total_price_usd: f64,
    pub performed_at: String,
    pub teeth: Vec<TreatmentTooth>,
    pub xrays: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatientVisitWithTreatments {
    pub visit_id: String,
    pub visit_date: String,
    pub chief_complaint: Option<String>,
    pub status: VisitStatus,
    pub procedures: Vec<TreatmentProcedure>,
}

// Procedure
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Procedure {
    pub id: String,
    pub name: String,
    pub additional_note: Option<String>,
    #[sqlx(default)]
    pub procedure_price_afn: f64,
    #[sqlx(default)]
    pub procedure_price_usd: f64,
    pub created_at: String,
    pub updated_at: String,
}

// Invoice
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: String,
    pub visit_id: String,
    pub invoice_number: String,
    #[sqlx(default)]
    pub subtotal_afn: f64,
    #[sqlx(default)]
    pub subtotal_usd: f64,
    #[sqlx(default)]
    pub discount_afn: f64,
    #[sqlx(default)]
    pub discount_usd: f64,
    #[sqlx(default)]
    pub total_afn: f64,
    #[sqlx(default)]
    pub total_usd: f64,
    #[sqlx(default)]
    pub paid_afn: f64,
    #[sqlx(default)]
    pub paid_usd: f64,
    #[sqlx(default)]
    pub outstanding_afn: f64,
    #[sqlx(default)]
    pub outstanding_usd: f64,
    pub status: InvoiceStatus,
    pub issued_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
pub enum InvoiceStatus {
    Unpaid,
    Partial,
    Paid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct InvoiceItem {
    pub id: i64,
    pub invoice_id: String,
    pub treatment_record_id: Option<String>,
    pub procedure_name: String,
    pub quantity: i32,
    #[sqlx(default)]
    pub unit_price_afn: f64,
    #[sqlx(default)]
    pub unit_price_usd: f64,
    #[sqlx(default)]
    pub total_price_afn: f64,
    #[sqlx(default)]
    pub total_price_usd: f64,
}

// Payment
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: String,
    pub invoice_id: String,
    pub amount_afn: f64,
    pub amount_usd: f64,
    pub method: Option<String>,
    pub notes: Option<String>,
    pub received_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreBackupInput {
    pub backup_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreBackupResult {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GDriveBackupFile {
    pub id: String,
    pub name: String,
    pub size: Option<i64>,
    pub modified_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreGDriveFileInput {
    pub file_id: String,
    pub file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreLocalFileInput {
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupValidation {
    pub valid: bool,
    pub file_size: i64,
    pub table_count: i64,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptPatient {
    pub id: String,
    pub full_name: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptPayment {
    pub id: String,
    pub invoice_id: String,
    pub amount_afn: f64,
    pub amount_usd: f64,
    pub method: Option<String>,
    pub notes: Option<String>,
    pub received_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptProcedure {
    pub treatment_record_id: String,
    pub procedure_name: String,
    pub additional_note: Option<String>,
    pub quantity: i32,
    #[sqlx(default)]
    pub unit_price_afn: f64,
    #[sqlx(default)]
    pub unit_price_usd: f64,
    #[sqlx(default)]
    pub total_price_afn: f64,
    #[sqlx(default)]
    pub total_price_usd: f64,
    pub performed_at: String,
    pub tooth_numbers: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptClinic {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptData {
    pub id: String,
    pub invoice_number: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_phone: Option<String>,
    pub visit_id: String,
    pub issue_date: String,
    pub currency: String,
    pub subtotal_afn: f64,
    pub subtotal_usd: f64,
    pub discount_afn: f64,
    pub discount_usd: f64,
    pub total_afn: f64,
    pub total_usd: f64,
    pub paid_afn: f64,
    pub paid_usd: f64,
    pub outstanding_afn: f64,
    pub outstanding_usd: f64,
    pub status: InvoiceStatus,
    pub procedures: Vec<ReceiptProcedure>,
    pub payments: Vec<ReceiptPayment>,
    pub clinic: ReceiptClinic,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
pub enum PaymentMethod {
    Cash,
    Card,
    Mobile,
    Insurance,
}

// X-ray
#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Xray {
    pub id: String,
    pub patient_id: String,
    pub treatment_record_id: Option<String>,
    pub file_path: String,
    pub is_primary: bool,
    pub uploaded_at: String,
}

// App settings
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AppSettings {
    pub id: i32,
    pub clinic_name: Option<String>,
    pub clinic_phone: Option<String>,
    pub clinic_address: Option<String>,
    pub support_email: Option<String>,
    pub clinic_logo: Option<String>,
    pub language: Option<String>,
    pub auto_backup_enabled: bool,
    pub auto_backup_frequency: String,
    pub auto_backup_target: String,
    pub last_backup_at: Option<String>,
    pub gdrive_client_id: Option<String>,
    pub gdrive_connected: bool,
    pub gdrive_folder_id: Option<String>,
    pub local_backup_enabled: bool,
    pub local_backup_frequency: String,
    pub local_last_backup_at: Option<String>,
    pub local_next_scheduled_backup: Option<String>,
    pub gdrive_backup_enabled: bool,
    pub gdrive_backup_frequency: String,
    pub gdrive_connected_email: Option<String>,
    pub gdrive_last_backup_at: Option<String>,
    pub gdrive_next_scheduled_backup: Option<String>,
    pub gdrive_last_sync_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// Reports
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyTrendPoint {
    pub day: String,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub active_patients: i64,
    pub total_visits_this_month: i64,
    pub revenue_this_month: f64,
    pub revenue_this_month_afn: f64,
    pub revenue_this_month_usd: f64,
    pub expenses_this_month: f64,
    pub expenses_this_month_afn: f64,
    pub expenses_this_month_usd: f64,
    pub outstanding_balance: f64,
    pub outstanding_balance_afn: f64,
    pub outstanding_balance_usd: f64,
    pub completed_visits_this_month: i64,
    pub cancelled_visits_this_month: i64,
    pub active_patients_trend: Vec<DailyTrendPoint>,
    pub visits_trend: Vec<DailyTrendPoint>,
    pub revenue_trend: Vec<DailyTrendPoint>,
    pub expenses_trend: Vec<DailyTrendPoint>,
    pub outstanding_trend: Vec<DailyTrendPoint>,
    pub prev_active_patients: i64,
    pub prev_total_visits: i64,
    pub prev_revenue: f64,
    pub prev_revenue_afn: f64,
    pub prev_revenue_usd: f64,
    pub prev_expenses: f64,
    pub prev_expenses_afn: f64,
    pub prev_expenses_usd: f64,
    pub prev_outstanding: f64,
    pub prev_outstanding_afn: f64,
    pub prev_outstanding_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyRevenuePoint {
    pub month: String,
    pub revenue: f64,
    pub revenue_afn: f64,
    pub revenue_usd: f64,
    pub expenses: f64,
    pub expenses_afn: f64,
    pub expenses_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilter {
    pub filter_type: String, // "daily", "weekly", "monthly", "custom"
    pub start_date: Option<String>, // "YYYY-MM-DD" for custom
    pub end_date: Option<String>,   // "YYYY-MM-DD" for custom
}

// Dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub daily_revenue: f64,
    pub daily_revenue_afn: f64,
    pub daily_revenue_usd: f64,
    pub patients_today: i64,
    pub outstanding_balance: f64,
    pub outstanding_balance_afn: f64,
    pub outstanding_balance_usd: f64,
    pub outstanding_invoices_count: i64,
    pub procedures_performed: i64,
    pub yesterday_revenue: f64,
    pub yesterday_revenue_afn: f64,
    pub yesterday_revenue_usd: f64,
    pub yesterday_patients: i64,
    pub yesterday_procedures: i64,
    pub daily_expenses: f64,
    pub daily_expenses_afn: f64,
    pub daily_expenses_usd: f64,
    pub yesterday_expenses: f64,
    pub yesterday_expenses_afn: f64,
    pub yesterday_expenses_usd: f64,
    pub net_today: f64,
    pub net_today_afn: f64,
    pub net_today_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientsFlowPoint {
    pub label: String,
    pub check_ins: i64,
    pub visits: i64,
    pub completed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcedureDistribution {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentPatient {
    pub id: String,
    pub full_name: String,
    pub phone: String,
    pub age: i32,
    pub gender: String,
    pub address: String,
    pub visit_date: String,
    pub status: String,
    pub visit_id: String,
}

// Input DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePatientInput {
    pub full_name: String,
    pub phone: String,
    pub age: i32,
    pub gender: Gender,
    pub address: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<Vec<String>>,
    pub visit_date: Option<String>,
    pub chief_complaint: Option<String>,
    pub procedures: Option<Vec<CreateProcedureWithTreatmentInput>>,
    pub discount: Option<f64>,
    pub discount_afn: Option<f64>,
    pub discount_usd: Option<f64>,
    // pub paid_amount: Option<f64>,
    pub paid_amount_afn: f64,
    pub paid_amount_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProcedureWithTreatmentInput {
    pub procedure_name: String,
    pub procedure_additional_note: Option<String>,
    pub procedure_price_afn: f64,
    pub procedure_price_usd: f64,
    pub number_of_procedures: i32,
    pub treatment_teeth: Option<Vec<TreatmentToothInput>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePatientInput {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub allergies: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePatientMedicalInfoInput {
    pub allergies: Option<String>,
    pub medical_conditions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVisitInput {
    pub patient_id: String,
    pub visit_date: Option<String>,
    pub chief_complaint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProcedureInput {
    pub visit_id: String,
    pub name: String,
    pub additional_note: Option<String>,
    pub procedure_price_afn: f64,
    pub procedure_price_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTreatmentRecordInput {
    pub visit_id: String,
    pub procedure_id: String,
    pub treatment_teeth: Vec<TreatmentToothInput>,
    pub number_of_procedures: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TreatmentToothInput {
    pub tooth_number: i32,
    pub tooth_quadrant: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceInput {
    pub visit_id: String,
    pub discount_afn: f64,
    pub discount_usd: f64,
    pub paid_amount_afn: f64,
    pub paid_amount_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPaymentInput {
    pub invoice_id: String,
    pub amount_afn: f64,
    pub amount_usd: f64,
    pub method: PaymentMethod,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InvoiceListItem {
    pub id: String,
    pub invoice_number: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_phone: Option<String>,
    pub visit_id: String,
    pub visit_date: String,
    #[sqlx(default)]
    pub subtotal_afn: f64,
    #[sqlx(default)]
    pub subtotal_usd: f64,
    #[sqlx(default)]
    pub discount_afn: f64,
    #[sqlx(default)]
    pub discount_usd: f64,
    #[sqlx(default)]
    pub total_afn: f64,
    #[sqlx(default)]
    pub total_usd: f64,
    #[sqlx(default)]
    pub paid_afn: f64,
    #[sqlx(default)]
    pub paid_usd: f64,
    #[sqlx(default)]
    pub outstanding_afn: f64,
    #[sqlx(default)]
    pub outstanding_usd: f64,
    pub status: String,
    pub issued_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePageResult {
    pub items: Vec<InvoiceListItem>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: i64,
    pub unpaid_count: i64,
    pub partial_count: i64,
    pub paid_count: i64,
    pub total_outstanding: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceListParams {
    pub query: Option<String>,
    pub status: Option<String>,
    pub patient_id: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// Expenses
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExpenseCategory {
    pub id: String,
    pub name: String,
    pub name_ps: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Expense {
    pub id: String,
    pub category_id: String,
    pub description: String,
    #[sqlx(default)]
    pub amount_afn: f64,
    #[sqlx(default)]
    pub amount_usd: f64,
    pub expense_date: String,
    pub payment_method: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExpenseListItem {
    pub id: String,
    pub category_id: String,
    pub category_name: String,
    pub category_name_ps: String,
    pub description: String,
    #[sqlx(default)]
    pub amount_afn: f64,
    #[sqlx(default)]
    pub amount_usd: f64,
    pub expense_date: String,
    pub payment_method: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpensePageResult {
    pub items: Vec<ExpenseListItem>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: i64,
    pub total_spent_afn: f64,
    pub total_spent_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseListParams {
    pub query: Option<String>,
    pub category_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExpenseInput {
    pub category_id: String,
    pub description: String,
    pub amount_afn: f64,
    pub amount_usd: f64,
    pub expense_date: String,
    pub payment_method: PaymentMethod,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateExpenseInput {
    pub category_id: String,
    pub description: String,
    pub amount_afn: f64,
    pub amount_usd: f64,
    pub expense_date: String,
    pub payment_method: PaymentMethod,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientPageResult {
    pub items: Vec<Patient>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientListParams {
    pub query: Option<String>,
    pub gender: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// Search result types for global search
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchPatient {
    pub id: String,
    pub full_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchInvoice {
    pub id: String,
    pub invoice_number: String,
    pub status: String,
    pub outstanding_afn: f64,
    pub outstanding_usd: f64,
    pub patient_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchReceipt {
    pub id: String,
    pub invoice_number: String,
    pub status: String,
    pub patient_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchVisit {
    pub id: String,
    pub visit_date: String,
    pub chief_complaint: Option<String>,
    pub patient_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchTreatment {
    pub id: String,
    pub name: String,
    pub additional_note: Option<String>,
    #[sqlx(default)]
    pub procedure_price_afn: f64,
    #[sqlx(default)]
    pub procedure_price_usd: f64,
    pub visit_id: String,
    pub practitioner_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GlobalSearchPayment {
    pub id: String,
    #[sqlx(default)]
    pub amount_afn: f64,
    #[sqlx(default)]
    pub amount_usd: f64,
    pub received_at: String,
    pub patient_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub result_type: String,
    pub title: String,
    pub subtitle: String,
    pub route: Option<String>,
}

// Tauri state
#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: AppConfig,
}

// Backup / Restore
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BackupRecord {
    pub id: i64,
    pub backup_type: String,
    pub backup_path: String,
    pub cloud_provider: String,
    pub status: String,
    pub file_size: i64,
    pub error_message: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupSettings {
    pub auto_backup_enabled: bool,
    pub auto_backup_frequency: String,
    pub auto_backup_target: String,
    pub last_backup_at: Option<String>,
    pub gdrive_client_id: Option<String>,
    pub gdrive_connected: bool,
    pub gdrive_folder_id: Option<String>,
    pub local_backup_enabled: bool,
    pub local_backup_frequency: String,
    pub local_last_backup_at: Option<String>,
    pub local_next_scheduled_backup: Option<String>,
    pub gdrive_backup_enabled: bool,
    pub gdrive_backup_frequency: String,
    pub gdrive_connected_email: Option<String>,
    pub gdrive_last_backup_at: Option<String>,
    pub gdrive_next_scheduled_backup: Option<String>,
    pub gdrive_last_sync_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBackupSettingsInput {
    pub auto_backup_enabled: Option<bool>,
    pub auto_backup_frequency: Option<String>,
    pub auto_backup_target: Option<String>,
    pub gdrive_client_id: Option<String>,
    pub local_backup_enabled: Option<bool>,
    pub local_backup_frequency: Option<String>,
    pub gdrive_backup_enabled: Option<bool>,
    pub gdrive_backup_frequency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartAuthResult {
    pub auth_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GDriveStatus {
    pub connected: bool,
    pub email: Option<String>,
    pub last_sync_at: Option<String>,
}

// All Visits list types
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VisitListItem {
    pub id: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_phone: Option<String>,
    pub visit_date: String,
    pub chief_complaint: Option<String>,
    pub status: String,
    pub procedures_count: i64,
    pub total_afn: f64,
    pub total_usd: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitPageResult {
    pub items: Vec<VisitListItem>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: i64,
    pub open_count: i64,
    pub completed_count: i64,
    pub cancelled_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitListParams {
    pub query: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// Auth types
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub recovery_key: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SessionRow {
    pub token: String,
    pub user_id: String,
    pub expires_at: String,
    pub created_at: String,
    #[sqlx(default)]
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordInput {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordInput {
    pub recovery_key: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetupResponse {
    pub token: String,
    pub username: String,
    pub user_id: String,
    pub recovery_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HasUsersResponse {
    pub has_users: bool,
}
