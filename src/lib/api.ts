import { invoke } from "@tauri-apps/api/core";
import type {
  Patient,
  CreatedPatient,
  PatientPageResult,
  CreatePatientInput,
  UpdatePatientInput,
  UpdatePatientMedicalInfoInput,
  Visit,
  CreateVisitInput,
  TreatmentRecord,
  CreateTreatmentRecordInput,
  Procedure,
  CreateProcedureInput,
  Invoice,
  InvoicePageResult,
  InvoiceListParams,
  Payment,
  AddPaymentInput,
  Expense,
  ExpenseCategory,
  ExpenseListParams,
  ExpensePageResult,
  CreateExpenseInput,
  UpdateExpenseInput,
  MonthlyRevenuePoint,
  ReportSummary,
  ReportFilter,
  AppSettings,
  UpdateSettingsInput,
  Xray,
  PatientMedicalInfo,
  PatientStatisticsInfo,
  PatientVisitWithTreatments,
  ReceiptData,
  CreateInvoiceInput,
  VisitPageResult,
  VisitListParams,
  AuthResponse,
  HasUsersResponse,
  LoginInput,
  SetupUserInput,
  ChangePasswordInput,
  ResetPasswordInput,
  SetupResponse,
} from "../types/ApiTypes";

import { SearchResult } from "../types/SearchTypes";
import type {
  DashboardStats,
  PatientsFlowPoint,
  RecentPatient,
  BackupRecord,
  BackupSettings,
  UpdateBackupSettingsInput,
  StartAuthResult,
  GDriveStatus,
  RestoreBackupInput,
  RestoreBackupResult,
  BackupValidation,
  GDriveBackupFile,
  RestoreGDriveFileInput,
} from "../types/ApiTypes";

export const api = {
  auth: {
    hasUsers: () => invoke<HasUsersResponse>("has_users"),
    setup: (input: SetupUserInput) =>
      invoke<SetupResponse>("setup_user", { input }),
    login: (input: LoginInput, rememberMe: boolean) =>
      invoke<AuthResponse>("login", { input, rememberMe }),
    verifySession: (token: string) =>
      invoke<AuthResponse>("verify_session", { token }),
    logout: (token: string) => invoke<void>("logout", { token }),
    changePassword: (userId: string, input: ChangePasswordInput) =>
      invoke<void>("change_password", { userId, input }),
    resetPassword: (input: ResetPasswordInput) =>
      invoke<AuthResponse>("reset_password", { input }),
  },
  patients: {
    list: (params: {
      query?: string;
      gender?: string;
      page?: number;
      perPage?: number;
    }) => invoke<PatientPageResult>("list_patients", { params }),
    get: (id: string) => invoke<Patient>("get_patient", { id }),
    getMedicalInfo: (id: string) =>
      invoke<PatientMedicalInfo>("get_patient_medical_info", { id }),
    getStatistics: (id: string) =>
      invoke<PatientStatisticsInfo>("get_patient_statistics", { id }),
    create: (input: CreatePatientInput) =>
      invoke<CreatedPatient>("create_patient", { input }),
    update: (id: string, input: UpdatePatientInput) =>
      invoke<Patient>("update_patient", { id, input }),
    updateMedicalInfo: (
      patient_id: string,
      input: UpdatePatientMedicalInfoInput,
    ) =>
      invoke<void>("update_patient_medical_info", {
        patientId: patient_id,
        input,
      }),
    delete: (id: string) => invoke<void>("delete_patient", { id }),
    add_medical_condition: (patient_id: string, condition_name: string) =>
      invoke<void>("add_medical_condition", {
        patientId: patient_id,
        conditionName: condition_name,
      }),
    upload_xray: (
      patient_id: string,
      treatment_record_id: string | null,
      filename: string,
      bytes: number[],
    ) =>
      invoke<Xray>("upload_xray", {
        patientId: patient_id,
        treatmentRecordId: treatment_record_id,
        filename,
        bytes,
      }),
    getXrays: (patient_id: string) =>
      invoke<Xray[]>("get_patient_xrays", { patientId: patient_id }),
  },
  invoices: {
    list: (params: InvoiceListParams) =>
      invoke<InvoicePageResult>("list_invoices", { params }),
    get: (id: string) => invoke<Invoice>("get_invoice", { id }),
    create: (input: CreateInvoiceInput) =>
      invoke<Invoice>("create_invoice", { input }),
    getForVisit: (visitId: string) =>
      invoke<Invoice | null>("get_visit_invoice", { visitId }),
    getPayments: (invoiceId: string) =>
      invoke<Payment[]>("get_invoice_payments", { invoiceId }),
  },
  visits: {
    create: (input: CreateVisitInput) =>
      invoke<Visit>("create_visit", { input }),
    updateStatus: (id: string, status: Visit["status"]) =>
      invoke<Visit>("update_visit_status", { id, status }),
    list: (patientId: string) =>
      invoke<Visit[]>("get_patient_visits", { patientId }),
    listAll: (params: VisitListParams) =>
      invoke<VisitPageResult>("list_all_visits", { params }),
    getWithTreatments: (patientId: string) =>
      invoke<PatientVisitWithTreatments[]>("get_patient_treatment_history", {
        patientId,
      }),
  },
  treatments: {
    add: (input: CreateTreatmentRecordInput) =>
      invoke<TreatmentRecord>("add_treatment_record", { input }),
  },
  receipts: {
    getByInvoiceId: (invoiceId: string) =>
      invoke<ReceiptData>("get_receipt_details", { invoiceId }),
    getByVisitId: (visitId: string) =>
      invoke<ReceiptData>("get_receipt_details_by_visit", { visitId }),
  },
  payments: {
    add: (input: AddPaymentInput) => invoke<Payment>("add_payment", { input }),
  },
  expenses: {
    list: (params: ExpenseListParams) =>
      invoke<ExpensePageResult>("list_expenses", { params }),
    get: (id: string) => invoke<Expense>("get_expense", { id }),
    create: (input: CreateExpenseInput) =>
      invoke<Expense>("create_expense", { input }),
    update: (id: string, input: UpdateExpenseInput) =>
      invoke<Expense>("update_expense", { id, input }),
    delete: (id: string) => invoke<void>("delete_expense", { id }),
    categories: () => invoke<ExpenseCategory[]>("list_expense_categories"),
  },
  procedures: {
    list: () => invoke<Procedure[]>("list_procedures"),
    findByName: (name: string) =>
      invoke<Procedure | null>("find_procedure_by_name", { name }),
    create: (input: CreateProcedureInput) =>
      invoke<Procedure>("create_procedure", { input }),
    updateAdditionalNote: (id: string, additionalNote: string | null) =>
      invoke<void>("update_procedure_additional_note", { id, additionalNote }),
  },
  reports: {
    summary: (filter: ReportFilter) => invoke<ReportSummary>("get_report_summary", { filter }),
    monthlyRevenue: (filter: ReportFilter) => invoke<MonthlyRevenuePoint[]>("get_monthly_revenue", { filter }),
  },
  settings: {
    get: () => invoke<AppSettings>("get_settings"),
    update: (input: UpdateSettingsInput) =>
      invoke<AppSettings>("update_settings", { input }),
  },
  search: {
    globalSearch: (query: string) =>
      invoke<SearchResult[]>("global_search", { query }),
  },
  dashboard: {
    stats: () => invoke<DashboardStats>("get_dashboard_stats"),
    patientsFlow: (mode: string) =>
      invoke<PatientsFlowPoint[]>("get_patients_flow", { mode }),
    recentPatients: () => invoke<RecentPatient[]>("get_recent_patients"),  },
  backups: {
    list: () => invoke<BackupRecord[]>("list_backups"),
    backupNow: (target: string, savePath?: string) =>
      invoke<BackupRecord[]>("backup_now", {
        target,
        save_path: savePath ?? null,
      }),
    delete: (id: number) => invoke<void>("delete_backup", { id }),
    getSettings: () => invoke<BackupSettings>("get_backup_settings"),
    updateSettings: (input: UpdateBackupSettingsInput) =>
      invoke<BackupSettings>("update_backup_settings", { input }),
    startGdriveAuth: () => invoke<StartAuthResult>("start_gdrive_auth"),
    getGdriveStatus: () => invoke<GDriveStatus>("get_gdrive_status"),
    disconnectGdrive: () => invoke<void>("disconnect_gdrive"),
    updateGdriveConnection: (email: string) =>
      invoke<void>("update_gdrive_connection", { email }),
    restore: (input: RestoreBackupInput) =>
      invoke<RestoreBackupResult>("restore_from_backup", { input }),
    validateFile: (path: string) =>
      invoke<BackupValidation>("validate_backup_file", { path }),
    getAvailableFiles: () =>
      invoke<BackupRecord[]>("get_available_backup_files"),
    listGdriveFiles: () =>
      invoke<GDriveBackupFile[]>("list_gdrive_backup_files"),
    restoreGdriveFile: (input: RestoreGDriveFileInput) =>
      invoke<RestoreBackupResult>("restore_gdrive_file", { input }),
    restoreLocalFile: (input: { file_path: string }) =>
      invoke<RestoreBackupResult>("restore_local_file", { input }),
    copyDbTo: (destPath: string) =>
      invoke<string>("copy_db_to", { destPath }),
  },
};
