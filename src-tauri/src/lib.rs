mod config;
mod crypto;
mod db;
mod models;
mod services;
mod utils;

use crate::config::AppConfig;
use crate::models::*;
use crate::services::*;
use tauri::Manager;
use tauri::State;

// Patient commands
#[tauri::command]
async fn list_patients(
    state: State<'_, AppState>,
    params: PatientListParams,
) -> Result<PatientPageResult, String> {
    let query = params.query.as_deref();
    let gender = params.gender.as_deref();
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    PatientService::list(&state.db, query, gender, page, per_page)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_patient(
    state: State<'_, AppState>,
    input: CreatePatientInput,
) -> Result<CreatedPatient, String> {
    PatientService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient(state: State<'_, AppState>, id: String) -> Result<Patient, String> {
    PatientService::find(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_patient(
    state: State<'_, AppState>,
    id: String,
    input: UpdatePatientInput,
) -> Result<Patient, String> {
    PatientService::update(&state.db, &id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_patient(state: State<'_, AppState>, id: String) -> Result<(), String> {
    PatientService::delete(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_medical_condition(
    state: State<'_, AppState>,
    patient_id: String,
    condition_name: String,
) -> Result<(), String> {
    PatientService::add_medical_condition(&state.db, &patient_id, &condition_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient_medical_info(
    state: State<'_, AppState>,
    id: String,
) -> Result<PatientMedicalInfoResponse, String> {
    PatientService::get_medical_info(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient_statistics(
    state: State<'_, AppState>,
    id: String,
) -> Result<PatientStatisticsResponse, String> {
    PatientService::get_statistics(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_patient_medical_info(
    state: State<'_, AppState>,
    patient_id: String,
    input: UpdatePatientMedicalInfoInput,
) -> Result<(), String> {
    PatientService::update_medical_info(&state.db, &patient_id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient_xrays(
    state: State<'_, AppState>,
    patient_id: String,
) -> Result<Vec<Xray>, String> {
    XrayService::list_for_patient(&state.db, &patient_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn upload_xray(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    patient_id: String,
    treatment_record_id: Option<String>,
    filename: String,
    bytes: Vec<u8>,
) -> Result<Xray, String> {
    XrayService::upload(&state.db, &app, &patient_id, treatment_record_id.as_deref(), &filename, &bytes)
        .await
        .map_err(|e| e.to_string())
}

// Visit commands
#[tauri::command]
async fn list_all_visits(
    state: State<'_, AppState>,
    params: crate::models::VisitListParams,
) -> Result<crate::models::VisitPageResult, String> {
    let query = params.query.as_deref();
    let status = params.status.as_deref();
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    VisitService::list_all(&state.db, query, status, page, per_page)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_visit(
    state: State<'_, AppState>,
    input: CreateVisitInput,
) -> Result<Visit, String> {
    VisitService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_visit_status(
    state: State<'_, AppState>,
    id: String,
    status: VisitStatus,
) -> Result<Visit, String> {
    VisitService::update_status(&state.db, &id, status)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient_visits(
    state: State<'_, AppState>,
    patient_id: String,
) -> Result<Vec<Visit>, String> {
    VisitService::get_by_patient(&state.db, &patient_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patient_treatment_history(
    state: State<'_, AppState>,
    patient_id: String,
) -> Result<Vec<PatientVisitWithTreatments>, String> {
    VisitService::get_with_treatments(&state.db, &patient_id)
        .await
        .map_err(|e| e.to_string())
}

// Treatment commands
#[tauri::command]
async fn add_treatment_record(
    state: State<'_, AppState>,
    input: CreateTreatmentRecordInput,
) -> Result<TreatmentRecord, String> {
    TreatmentRecordService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

// Invoice commands
#[tauri::command]
async fn list_invoices(
    state: State<'_, AppState>,
    params: crate::models::InvoiceListParams,
) -> Result<crate::models::InvoicePageResult, String> {
    InvoiceService::list(&state.db, params)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_invoice(
    state: State<'_, AppState>,
    input: CreateInvoiceInput,
) -> Result<Invoice, String> {
    InvoiceService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_invoice(
    state: State<'_, AppState>,
    id: String,
) -> Result<Invoice, String> {
    InvoiceService::find(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_visit_invoice(
    state: State<'_, AppState>,
    visit_id: String,
) -> Result<Option<Invoice>, String> {
    InvoiceService::get_for_visit(&state.db, &visit_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_receipt_details(
    state: State<'_, AppState>,
    invoice_id: String,
) -> Result<ReceiptData, String> {
    InvoiceService::get_receipt_details(&state.db, &invoice_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_receipt_details_by_visit(
    state: State<'_, AppState>,
    visit_id: String,
) -> Result<ReceiptData, String> {
    InvoiceService::get_receipt_details_by_visit(&state.db, &visit_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_payment(
    state: State<'_, AppState>,
    input: AddPaymentInput,
) -> Result<Payment, String> {
    PaymentService::add(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_invoice_payments(
    state: State<'_, AppState>,
    invoice_id: String,
) -> Result<Vec<Payment>, String> {
    PaymentService::list_for_invoice(&state.db, &invoice_id)
        .await
        .map_err(|e| e.to_string())
}

// Expense commands
#[tauri::command]
async fn list_expenses(
    state: State<'_, AppState>,
    params: ExpenseListParams,
) -> Result<ExpensePageResult, String> {
    ExpenseService::list(&state.db, params)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_expense(state: State<'_, AppState>, id: String) -> Result<Expense, String> {
    ExpenseService::find(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_expense(
    state: State<'_, AppState>,
    input: CreateExpenseInput,
) -> Result<Expense, String> {
    ExpenseService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_expense(
    state: State<'_, AppState>,
    id: String,
    input: UpdateExpenseInput,
) -> Result<Expense, String> {
    ExpenseService::update(&state.db, &id, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_expense(state: State<'_, AppState>, id: String) -> Result<(), String> {
    ExpenseService::delete(&state.db, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_expense_categories(
    state: State<'_, AppState>,
) -> Result<Vec<ExpenseCategory>, String> {
    ExpenseService::list_categories(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn global_search(
    state: State<'_, AppState>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    SearchService::global_search(&state.db, &query)
        .await
        .map_err(|e| e.to_string())
}

// Reports commands
#[tauri::command]
async fn get_report_summary(state: State<'_, AppState>, filter: ReportFilter) -> Result<ReportSummary, String> {
    ReportService::summary(&state.db, &filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_monthly_revenue(state: State<'_, AppState>, filter: ReportFilter) -> Result<Vec<MonthlyRevenuePoint>, String> {
    ReportService::monthly_revenue(&state.db, &filter)
        .await
        .map_err(|e| e.to_string())
}

// Dashboard commands
#[tauri::command]
async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats, String> {
    DashboardService::stats(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_patients_flow(
    state: State<'_, AppState>,
    mode: String,
) -> Result<Vec<PatientsFlowPoint>, String> {
    DashboardService::patients_flow(&state.db, &mode)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_procedure_distribution(
    state: State<'_, AppState>,
    mode: String,
) -> Result<Vec<ProcedureDistribution>, String> {
    DashboardService::procedure_distribution(&state.db, &mode)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_recent_patients(
    state: State<'_, AppState>,
) -> Result<Vec<RecentPatient>, String> {
    DashboardService::recent_patients(&state.db)
        .await
        .map_err(|e| e.to_string())
}

// Procedure commands
#[tauri::command]
async fn create_procedure(
    state: State<'_, AppState>,
    input: CreateProcedureInput,
) -> Result<Procedure, String> {
    ProcedureService::create(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_procedures(state: State<'_, AppState>) -> Result<Vec<Procedure>, String> {
    ProcedureService::list(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn find_procedure_by_name(
    state: State<'_, AppState>,
    name: String,
) -> Result<Option<Procedure>, String> {
    ProcedureService::find_by_name(&state.db, &name)
        .await
        .map_err(|e| e.to_string())
}

// Settings commands
#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    SettingsService::get(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_settings(
    state: State<'_, AppState>,
    input: UpdateSettingsInput,
) -> Result<AppSettings, String> {
    SettingsService::update(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

// Backup commands
#[tauri::command]
async fn list_backups(state: State<'_, AppState>) -> Result<Vec<BackupRecord>, String> {
    BackupService::list_backups(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn backup_now(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    target: String,
    save_path: Option<String>,
) -> Result<Vec<BackupRecord>, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let backup_dir = app_data.join("backups");
    let mut results = Vec::new();

    let do_local = target == "local" || target == "both";
    let do_gdrive = target == "google_drive" || target == "both";

    let custom_path = if do_local { save_path.as_deref() } else { None };

    let local_file_path = if do_local || (do_gdrive && target == "both") {
        let record = BackupService::create_local_backup(
            &state.db, &backup_dir, "manual", custom_path,
        ).await
            .map_err(|e| e.to_string())?;
        let path = record.backup_path.clone();
        results.push(record);
        Some(path)
    } else if do_gdrive {
        let now = chrono::Utc::now();
        let temp_filename = format!("dental_clinic_tmp_{}.db", now.format("%Y%m%d_%H%M%S"));
        let temp_path = std::env::temp_dir().join(&temp_filename);
        let temp_dir_parent = temp_path.parent().unwrap().to_path_buf();
        std::fs::create_dir_all(&temp_dir_parent).map_err(|e| e.to_string())?;
        let dest_str = temp_path.to_string_lossy().replace('\\', "/");
        let escaped = dest_str.replace('\'', "''");
        let sql = format!("VACUUM INTO '{}'", escaped);
        sqlx::query(&sql).execute(&state.db).await
            .map_err(|e| format!("VACUUM INTO failed: {}", e))?;
        Some(temp_path.to_string_lossy().to_string())
    } else {
        None
    };

    if do_gdrive {
        let settings = BackupService::get_backup_settings(&state.db).await
            .map_err(|e| e.to_string())?;
        let client_id = settings.gdrive_client_id.as_deref()
            .ok_or_else(|| "Google Drive not configured. Add a Client ID in Settings.".to_string())?;
        let client_secret = state.config.google_oauth_client_secret.as_str();
        let app_data_path = app_data.clone();

        match GDriveClient::ensure_valid_token(client_id, Some(client_secret), &app_data_path).await {
            Ok(access_token) => {
                let folder_id = match &settings.gdrive_folder_id {
                    Some(id) if !id.is_empty() => id.clone(),
                    _ => {
                        match GDriveClient::ensure_folder(&access_token, "Dental Clinic Backups").await {
                            Ok(id) => {
                                sqlx::query("UPDATE app_settings SET gdrive_folder_id = ? WHERE id = 1")
                                    .bind(&id)
                                    .execute(&state.db).await.ok();
                                id
                            }
                            Err(e) => {
                                results.push(BackupService::record_failed_backup(
                                    &state.db, "manual", "google_drive", &e.to_string()
                                ).await.map_err(|e| e.to_string())?);
                                BackupService::set_last_backup_now(&state.db).await.ok();
                                return Ok(results);
                            }
                        }
                    }
                };

                let file_path = local_file_path.as_ref().ok_or_else(|| "missing backup file".to_string())?;
                let now_str = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                let drive_filename = format!("dental_clinic_{}.db", now_str);

                let bytes = std::fs::read(file_path)
                    .map_err(|e| format!("read backup file: {}", e))?;

                match GDriveClient::upload_file(&access_token, &folder_id, &drive_filename, &bytes).await {
                    Ok(file_id) => {
                        let finished = chrono::Utc::now().to_rfc3339();
                        let record = sqlx::query_as::<_, BackupRecord>(
                            "INSERT INTO backups (backup_type, backup_path, cloud_provider, status, file_size, created_at, completed_at)
                             VALUES (?, ?, 'google_drive', 'success', ?, datetime('now'), ?)
                             RETURNING id, backup_type, backup_path, cloud_provider, status, file_size, error_message, created_at, completed_at"
                        )
                        .bind("manual")
                        .bind(format!("gdrive:{}", file_id))
                        .bind(bytes.len() as i64)
                        .bind(&finished)
                        .fetch_one(&state.db)
                        .await
                        .map_err(|e| e.to_string())?;
                        results.push(record);
                    }
                    Err(e) => {
                        results.push(BackupService::record_failed_backup(
                            &state.db, "manual", "google_drive", &e.to_string()
                        ).await.map_err(|e| e.to_string())?);
                    }
                }

                if target == "google_drive" {
                    if let Some(path) = &local_file_path {
                        std::fs::remove_file(path).ok();
                    }
                }
            }
            Err(e) => {
                results.push(BackupService::record_failed_backup(
                    &state.db, "manual", "google_drive", &e.to_string()
                ).await.map_err(|e| e.to_string())?);
            }
        }
    }

    if do_local {
        BackupService::set_local_backup_now(&state.db).await.ok();
    }
    if do_gdrive {
        BackupService::set_gdrive_backup_now(&state.db).await.ok();
    }
    BackupService::set_last_backup_now(&state.db).await.ok();
    Ok(results)
}

#[tauri::command]
async fn copy_db_to(app: tauri::AppHandle, dest_path: String) -> Result<String, String> {
    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("dental_clinic.db");

    let dest = std::path::Path::new(&dest_path);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    std::fs::copy(&db_path, &dest)
        .map_err(|e| format!("Failed to copy database file: {}", e))?;

    Ok(dest_path)
}

#[tauri::command]
async fn delete_backup(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    id: i64,
) -> Result<(), String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let record = sqlx::query_as::<_, BackupRecord>(
        "SELECT id, backup_type, backup_path, cloud_provider, status, file_size, error_message, created_at, completed_at
         FROM backups WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    if let Some(r) = record {
        if r.cloud_provider == "local" {
            let path = std::path::Path::new(&r.backup_path);
            if path.exists() {
                std::fs::remove_file(path).ok();
            }
        } else if let Some(file_id) = r.backup_path.strip_prefix("gdrive:") {
            let settings = BackupService::get_backup_settings(&state.db).await?;
            let client_id = settings.gdrive_client_id
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| state.config.google_oauth_client_id.clone());
            if client_id.is_empty() {
                return Err("Google Drive client ID not configured".to_string());
            }

            let token = GDriveClient::ensure_valid_token(
                &client_id,
                Some(state.config.google_oauth_client_secret.as_str()),
                &app_data,
            )
            .await?;
            GDriveClient::delete_file(&token, file_id).await?;
        }

        sqlx::query("DELETE FROM backups WHERE id = ?")
            .bind(id)
            .execute(&state.db)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn get_backup_settings(state: State<'_, AppState>) -> Result<BackupSettings, String> {
    BackupService::get_backup_settings(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_backup_settings(
    state: State<'_, AppState>,
    input: UpdateBackupSettingsInput,
) -> Result<BackupSettings, String> {
    BackupService::update_backup_settings(&state.db, &input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_gdrive_auth(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<StartAuthResult, String> {
    let settings = BackupService::get_backup_settings(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    let client_id = settings.gdrive_client_id
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| state.config.google_oauth_client_id.clone());
    let client_secret = state.config.google_oauth_client_secret.as_str();
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let auth_url = GDriveClient::start_auth_flow(&client_id, Some(client_secret), app_data, app)
        .await
        .map_err(|e| e.to_string())?;

    Ok(StartAuthResult { auth_url })
}

#[tauri::command]
async fn update_gdrive_connection(
    state: State<'_, AppState>,
    email: String,
) -> Result<(), String> {
    sqlx::query(
        "UPDATE app_settings SET gdrive_connected = 1, gdrive_connected_email = ?, gdrive_last_sync_at = datetime('now') WHERE id = 1"
    )
    .bind(&email)
    .execute(&state.db)
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_gdrive_status(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<GDriveStatus, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let connected = GDriveClient::load_token(&app_data).is_some();
    let settings = BackupService::get_backup_settings(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(GDriveStatus {
        connected,
        email: settings.gdrive_connected_email,
        last_sync_at: settings.gdrive_last_sync_at,
    })
}

#[tauri::command]
async fn disconnect_gdrive(
    app: tauri::AppHandle,
) -> Result<(), String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    GDriveClient::delete_token(&app_data);
    Ok(())
}

// Restore commands
#[tauri::command]
async fn restore_from_backup(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    input: crate::models::RestoreBackupInput,
) -> Result<crate::models::RestoreBackupResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let db_path = app_data.join("dental_clinic.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    // Fetch the backup record so we can handle local and Google Drive backups
    let record = sqlx::query_as::<_, crate::models::BackupRecord>(
        "SELECT id, backup_type, backup_path, cloud_provider, status, file_size, error_message, created_at, completed_at
         FROM backups WHERE id = ? AND status = 'success'"
    )
    .bind(input.backup_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| e.to_string())?;

    let record = record.ok_or_else(|| "Backup not found".to_string())?;

    let is_gdrive = record.cloud_provider == "google_drive";

    let backup_file_path = if record.cloud_provider == "local" {
        BackupService::get_backup_file_path(&state.db, input.backup_id).await
            .map_err(|e| e.to_string())?
    } else if record.cloud_provider == "google_drive" {
        let file_id = record.backup_path.strip_prefix("gdrive:")
            .ok_or_else(|| "Invalid Google Drive backup path".to_string())?;

        let settings = BackupService::get_backup_settings(&state.db).await
            .map_err(|e| e.to_string())?;
        let client_id = settings.gdrive_client_id
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| state.config.google_oauth_client_id.clone());

        if client_id.is_empty() {
            return Err("Google Drive client ID not configured".to_string());
        }

        let access_token = GDriveClient::ensure_valid_token(
            &client_id,
            Some(state.config.google_oauth_client_secret.as_str()),
            &app_data,
        )
        .await
        .map_err(|e| e.to_string())?;

        // Download to a temporary location (requirement 3)
        let tmp_dir = app_data.join("gdrive_restores");
        std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;
        let filename = format!("gdrive_restore_{}_{}.db", input.backup_id, chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        let dest_path = tmp_dir.join(filename);

        GDriveClient::download_file(&access_token, file_id, &dest_path).await
            .map_err(|e| e.to_string())?;

        dest_path.to_string_lossy().to_string()
    } else {
        return Err("Unsupported backup provider".to_string());
    };

    let result = BackupService::restore_from_backup(
        &state.db,
        &db_path_str,
        &backup_file_path,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Preserve this backup record in the restored database
    let opts = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&db_path_str)
        .create_if_missing(false);
    if let Ok(temp_pool) = sqlx::SqlitePool::connect_with(opts).await {
        sqlx::query(
            "INSERT OR REPLACE INTO backups (id, backup_type, backup_path, cloud_provider, status, file_size, error_message, created_at, completed_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(record.id)
        .bind(&record.backup_type)
        .bind(&record.backup_path)
        .bind(&record.cloud_provider)
        .bind("success")
        .bind(record.file_size)
        .bind(&record.error_message)
        .bind(&record.created_at)
        .bind(&record.completed_at)
        .execute(&temp_pool)
        .await
        .ok();
        temp_pool.close().await;
    }

    // Clean up the downloaded temporary file for GDrive backups (requirement 9)
    if is_gdrive {
        std::fs::remove_file(&backup_file_path).ok();
        // Also try to clean up the temp directory if empty
        if let Some(parent) = std::path::Path::new(&backup_file_path).parent() {
            if parent.is_dir() && parent.read_dir().map(|mut d| d.next().is_none()).unwrap_or(false) {
                std::fs::remove_dir(parent).ok();
            }
        }
    }

    Ok(result)
}

#[tauri::command]
async fn validate_backup_file(
    path: String,
) -> Result<BackupValidation, String> {
    BackupService::validate_backup_file(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_available_backup_files(
    state: State<'_, AppState>,
) -> Result<Vec<BackupRecord>, String> {
    BackupService::list_available_backups(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_gdrive_backup_files(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<Vec<GDriveBackupFile>, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let settings = BackupService::get_backup_settings(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let client_id = settings.gdrive_client_id
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| state.config.google_oauth_client_id.clone());

    if client_id.is_empty() {
        return Err("Google Drive client ID not configured".to_string());
    }

    let access_token = GDriveClient::ensure_valid_token(
        &client_id,
        Some(state.config.google_oauth_client_secret.as_str()),
        &app_data,
    )
    .await
    .map_err(|e| e.to_string())?;

    let folder_id = settings.gdrive_folder_id
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "Google Drive backup folder not found".to_string())?;

    GDriveClient::list_files(&access_token, &folder_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn restore_gdrive_file(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    input: RestoreGDriveFileInput,
) -> Result<RestoreBackupResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let db_path = app_data.join("dental_clinic.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let settings = BackupService::get_backup_settings(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let client_id = settings.gdrive_client_id
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| state.config.google_oauth_client_id.clone());

    if client_id.is_empty() {
        return Err("Google Drive client ID not configured".to_string());
    }

    let access_token = GDriveClient::ensure_valid_token(
        &client_id,
        Some(state.config.google_oauth_client_secret.as_str()),
        &app_data,
    )
    .await
    .map_err(|e| e.to_string())?;

    let tmp_dir = app_data.join("gdrive_restores");
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;
    let filename = format!("gdrive_restore_{}.db", input.file_id);
    let dest_path = tmp_dir.join(filename);

    GDriveClient::download_file(&access_token, &input.file_id, &dest_path)
        .await
        .map_err(|e| e.to_string())?;

    let result = BackupService::restore_from_backup(
        &state.db,
        &db_path_str,
        &dest_path.to_string_lossy(),
    )
    .await
    .map_err(|e| e.to_string())?;

    std::fs::remove_file(&dest_path).ok();
    if let Some(parent) = dest_path.parent() {
        if parent.is_dir() && parent.read_dir().map(|mut d| d.next().is_none()).unwrap_or(false) {
            std::fs::remove_dir(parent).ok();
        }
    }

    Ok(result)
}

#[tauri::command]
async fn restore_local_file(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    input: RestoreLocalFileInput,
) -> Result<RestoreBackupResult, String> {
    let app_data = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    let db_path = app_data.join("dental_clinic.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let result = BackupService::restore_from_backup(
        &state.db,
        &db_path_str,
        &input.file_path,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(result)
}

async fn run_auto_backup(app: &tauri::AppHandle, pool: &sqlx::SqlitePool, config: &AppConfig) {
    let settings = match BackupService::get_backup_settings(pool).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Auto-backup: failed to get settings: {}", e);
            return;
        }
    };

    let app_data = match app.path().app_data_dir() {
        Ok(d) => d,
        Err(_) => return,
    };

    // Google Drive auto backup
    if settings.gdrive_backup_enabled {
        if BackupService::is_backup_due_for(&settings.gdrive_last_backup_at, &settings.gdrive_backup_frequency) {
            let backup_type = &settings.gdrive_backup_frequency;
            let client_id = settings.gdrive_client_id
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| config.google_oauth_client_id.clone());

            match GDriveClient::ensure_valid_token(&client_id, Some(config.google_oauth_client_secret.as_str()), &app_data).await {
                Ok(access_token) => {
                    let folder_id = match &settings.gdrive_folder_id {
                        Some(id) if !id.is_empty() => id.clone(),
                        _ => {
                            match GDriveClient::ensure_folder(&access_token, "Dental Clinic Backups").await {
                                Ok(id) => {
                                    sqlx::query("UPDATE app_settings SET gdrive_folder_id = ? WHERE id = 1")
                                        .bind(&id).execute(pool).await.ok();
                                    id
                                }
                                Err(e) => {
                                    eprintln!("Auto-backup (gdrive): folder: {}", e);
                                    BackupService::record_failed_backup(pool, backup_type, "google_drive", &e.to_string()).await.ok();
                                    return;
                                }
                            }
                        }
                    };

                    let now_str = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                    let drive_filename = format!("dental_clinic_{}.db", now_str);
                    let temp_path = std::env::temp_dir().join(&drive_filename);
                    let dest_str = temp_path.to_string_lossy().replace('\\', "/");
                    let escaped = dest_str.replace('\'', "''");

                    if let Err(e) = sqlx::query(&format!("VACUUM INTO '{}'", escaped)).execute(pool).await {
                        eprintln!("Auto-backup (gdrive): VACUUM INTO: {}", e);
                        BackupService::record_failed_backup(pool, backup_type, "google_drive", &e.to_string()).await.ok();
                        return;
                    }

                    let bytes = match std::fs::read(&temp_path) {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("Auto-backup (gdrive): read temp file: {}", e);
                            return;
                        }
                    };

                    match GDriveClient::upload_file(&access_token, &folder_id, &drive_filename, &bytes).await {
                        Ok(file_id) => {
                            let finished = chrono::Utc::now().to_rfc3339();
                            sqlx::query(
                                "INSERT INTO backups (backup_type, backup_path, cloud_provider, status, file_size, created_at, completed_at)
                                 VALUES (?, ?, 'google_drive', 'success', ?, datetime('now'), ?)"
                            )
                            .bind(backup_type)
                            .bind(format!("gdrive:{}", file_id))
                            .bind(bytes.len() as i64)
                            .bind(&finished)
                            .execute(pool).await.ok();
                            BackupService::set_gdrive_backup_now(pool).await.ok();
                        }
                        Err(e) => {
                            eprintln!("Auto-backup (gdrive) upload: {}", e);
                            BackupService::record_failed_backup(pool, backup_type, "google_drive", &e.to_string()).await.ok();
                        }
                    }

                    std::fs::remove_file(&temp_path).ok();
                }
                Err(e) => {
                    eprintln!("Auto-backup (gdrive): token: {}", e);
                    BackupService::record_failed_backup(pool, backup_type, "google_drive", &e.to_string()).await.ok();
                }
            }
        }
    }
}

// Auth commands
#[tauri::command]
async fn has_users(state: State<'_, AppState>) -> Result<HasUsersResponse, String> {
    let has = AuthService::has_users(&state.db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(HasUsersResponse { has_users: has })
}

#[tauri::command]
async fn setup_user(
    state: State<'_, AppState>,
    input: SetupUserInput,
) -> Result<SetupResponse, String> {
    AuthService::setup(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn login(
    state: State<'_, AppState>,
    input: LoginInput,
    remember_me: bool,
) -> Result<AuthResponse, String> {
    AuthService::login(&state.db, input, remember_me)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn verify_session(
    state: State<'_, AppState>,
    token: String,
) -> Result<AuthResponse, String> {
    AuthService::verify_session(&state.db, &token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn logout(
    state: State<'_, AppState>,
    token: String,
) -> Result<(), String> {
    AuthService::logout(&state.db, &token)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn reset_password(
    state: State<'_, AppState>,
    input: ResetPasswordInput,
) -> Result<AuthResponse, String> {
    AuthService::reset_password(&state.db, input)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn change_password(
    state: State<'_, AppState>,
    user_id: String,
    input: ChangePasswordInput,
) -> Result<(), String> {
    AuthService::change_password(&state.db, &user_id, input)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let config = tauri::async_runtime::block_on(async {
                AppConfig::load().map_err(|e| {
                    eprintln!("{}", e);
                    e
                })
            })?;

            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to resolve app data directory: {}", e))?;
            let db_path = app_data_dir
                .join("dental_clinic.db")
                .to_string_lossy()
                .to_string();

            // Migrate data from the old bundle identifier folder (com.dentix.app).
            // app_data_dir is derived from the identifier, so renaming it changes
            // where the DB and encryption key live.
            if let Some(parent) = app_data_dir.parent() {
                let legacy_dir = parent.join("com.dentix.app");
                let new_db = app_data_dir.join("dental_clinic.db");
                let legacy_db = legacy_dir.join("dental_clinic.db");
                if !new_db.exists() && legacy_db.exists() {
                    if let Err(e) = std::fs::create_dir_all(&app_data_dir) {
                        eprintln!("[WARN] Failed to create app data dir: {}", e);
                    } else if let Ok(entries) = std::fs::read_dir(&legacy_dir) {
                        for entry in entries.flatten() {
                            let target = app_data_dir.join(entry.file_name());
                            if let Err(e) = std::fs::rename(entry.path(), &target) {
                                eprintln!(
                                    "[WARN] Failed to migrate {}: {}",
                                    entry.path().display(),
                                    e
                                );
                            }
                        }
                        println!(
                            "[INFO] Migrated app data from {}",
                            legacy_dir.display()
                        );
                    }
                }
            }

            println!("[INFO] Database path: {}", db_path);

            let pool = tauri::async_runtime::block_on(db::init_pool(&db_path));
            match pool {
                Ok(pool) => {
                    if let Err(e) = tauri::async_runtime::block_on(db::run_migrations(&pool)) {
                        eprintln!("Failed to run migrations: {}", e);
                        return Err(e.into());
                    }
                    app.manage(AppState { db: pool.clone(), config: config.clone() });

                    let app_handle = app.handle().clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app_handle.state::<AppState>();
                        run_auto_backup(&app_handle, &state.db, &state.config).await;
                    });
                }
                Err(e) => {
                    eprintln!("Failed to init DB: {}", e);
                    return Err(e.into());
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_patients,
            create_patient,
            get_patient,
            update_patient,
            delete_patient,
            add_medical_condition,
            update_patient_medical_info,
            get_patient_medical_info,
            get_patient_statistics,
            get_patient_xrays,
            upload_xray,
            create_visit,
            list_all_visits,
            update_visit_status,
            get_patient_visits,
            get_patient_treatment_history,
            add_treatment_record,
            create_invoice,
            get_invoice,
            get_visit_invoice,
            list_invoices,
            get_receipt_details,
            get_receipt_details_by_visit,
            add_payment,
            get_invoice_payments,
            list_expenses,
            get_expense,
            create_expense,
            update_expense,
            delete_expense,
            list_expense_categories,
            create_procedure,
            list_procedures,
            find_procedure_by_name,
            get_report_summary,
            get_monthly_revenue,
            get_settings,
            update_settings,
            global_search,
            get_dashboard_stats,
            get_patients_flow,
            get_procedure_distribution,
            get_recent_patients,
            list_backups,
            backup_now,
            copy_db_to,
            delete_backup,
            get_backup_settings,
            update_backup_settings,
            start_gdrive_auth,
            get_gdrive_status,
            disconnect_gdrive,
            update_gdrive_connection,
            restore_from_backup,
            validate_backup_file,
            get_available_backup_files,
            list_gdrive_backup_files,
            restore_gdrive_file,
            restore_local_file,
            has_users,
            setup_user,
            login,
            verify_session,
            logout,
            change_password,
            reset_password,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
