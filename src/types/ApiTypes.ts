export interface Patient {
  id: string;
  full_name: string;
  phone: string;
  age: number;
  gender: "Male" | "Female" | "Other";
  address: string | null;
  created_at: string;
  updated_at: string;
  initials: string;
  last_visit?: string | null;
}

export interface CreatedPatient {
  id: string;
  full_name: string;
  phone: string;
  age: number;
  gender: "Male" | "Female" | "Other";
  address: string | null;
  created_at: string;
  updated_at: string;
  last_visit?: string | null;
  visit_id: string;
  invoice_id: string;
  invoice_number: string;
  treatment_record_id: string | null;
}

export interface PatientMedicalInfo {
  allergies: string[];
  medical_conditions: string[];
}

export interface PatientStatisticsInfo {
  total_spent: number;
  total_spent_afn: number;
  total_spent_usd: number;
  last_visit_date: string | null;
  last_visit_procedure: string | null;
  outstanding_balance: number;
  outstanding_balance_afn: number;
  outstanding_balance_usd: number;
}

export interface UpdatePatientMedicalInfoInput {
  allergies: string | null;
  medical_conditions: string[] | null;
}

export interface PatientPageResult {
  items: Patient[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface InvoicePageResult {
  items: InvoiceListItem[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
  unpaid_count: number;
  partial_count: number;
  paid_count: number;
  total_outstanding: number;
}

export interface InvoiceListItem {
  id: string;
  invoice_number: string;
  patient_id: string;
  patient_name: string;
  patient_phone: string | null;
  visit_id: string;
  visit_date: string;
  subtotal_afn: number;
  subtotal_usd: number;
  discount_afn: number;
  discount_usd: number;
  total_afn: number;
  total_usd: number;
  paid_afn: number;
  paid_usd: number;
  outstanding_afn: number;
  outstanding_usd: number;
  status: "Unpaid" | "Partial" | "Paid";
  issued_at: string;
}

export interface InvoiceListParams {
  query?: string;
  status?: string;
  patientId?: string;
  page?: number;
  perPage?: number;
}

export type InvoiceStatusFilter = "All" | "Unpaid" | "Partial" | "Paid";

export interface CreatePatientInput {
  full_name: string;
  phone: string;
  age: number;
  gender: "Male" | "Female" | "Other";
  address?: string | null;
  allergies?: string | null;
  medical_conditions?: string[] | null;
  visit_date?: string | null;
  chief_complaint?: string | null;
  procedures: CreateProcedureWithTreatmentInput[];
  discount_afn?: number | null;
  discount_usd?: number | null;
  paid_amount_afn: number;
  paid_amount_usd: number;
}

export interface CreateProcedureWithTreatmentInput {
  procedure_name: string;
  procedure_additional_note?: string | null;
  procedure_price_afn: number;
  procedure_price_usd: number;
  number_of_procedures: number;
  treatment_teeth: TreatmentToothInput[];
}

export interface UpdatePatientInput {
  full_name?: string;
  phone?: string;
  age?: number;
  gender?: "Male" | "Female" | "Other";
  address?: string | null;
  allergies?: string | null;
}

export interface Visit {
  id: string;
  patient_id: string;
  visit_date: string;
  chief_complaint?: string | null;
  status: "Open" | "Completed" | "Cancelled";
  created_at: string;
  updated_at: string;
}

export interface CreateVisitInput {
  patient_id: string;
  visit_date?: string;
  chief_complaint?: string | null;
}

export interface CreateProcedureInput {
  visit_id: string;
  name: string;
  additional_note?: string | null;
  procedure_price_afn: number;
  procedure_price_usd: number;
}

export interface TreatmentRecord {
  id: string;
  visit_id: string;
  procedure_id: string;
  number_of_procedures: number;
  performed_at: string;
}

export interface TreatmentTooth {
  id: number;
  treatment_record_id: string;
  tooth_number: number;
  tooth_quadrant: string;
}

export interface TreatmentProcedure {
  treatment_record_id: string;
  procedure_name: string;
  procedure_additional_note: string | null;
  number_of_procedures: number;
  unit_price_afn: number;
  unit_price_usd: number;
  total_price_afn: number;
  total_price_usd: number;
  performed_at: string;
  teeth: TreatmentTooth[];
  xrays: string[];
}

export interface PatientVisitWithTreatments {
  visit_id: string;
  visit_date: string;
  chief_complaint: string | null;
  status: "Open" | "Completed" | "Cancelled";
  procedures: TreatmentProcedure[];
}

export interface TreatmentToothInput {
  tooth_number: number;
  tooth_quadrant: string;
}

export interface CreateTreatmentRecordInput {
  visit_id: string;
  procedure_id: string;
  treatment_teeth: TreatmentToothInput[];
  number_of_procedures: number;
}

export interface Procedure {
  id: string;
  name: string;
  additional_note?: string | null;
  procedure_price_afn: number;
  procedure_price_usd: number;
  created_at: string;
  updated_at: string;
}

export interface Invoice {
  id: string;
  visit_id: string;
  invoice_number: string;
  subtotal_afn: number;
  subtotal_usd: number;
  discount_afn: number;
  discount_usd: number;
  total_afn: number;
  total_usd: number;
  paid_afn: number;
  paid_usd: number;
  outstanding_afn: number;
  outstanding_usd: number;
  status: "Unpaid" | "Partial" | "Paid";
  issued_at: string;
}

export interface CreateInvoiceInput {
    visit_id: string;
    discount_afn: number;
    discount_usd: number;
    paid_amount_afn: number;
    paid_amount_usd: number;
}

export interface Payment {
  id: string;
  invoice_id: string;
  amount: number;
  notes?: string | null;
  received_at: string;
}

export interface ReceiptClinic {
  name: string;
  address: string;
  phone: string;
  logoUrl?: string | null;
}

export interface ReceiptPatient {
  id: string;
  fullName: string;
  phone?: string | null;
}

export interface ReceiptPayment {
  id: string;
  amount: number;
  amountAfn: number;
  amountUsd: number;
  method?: string | null;
  notes?: string | null;
  receivedAt: string;
}

export interface ReceiptProcedure {
  treatmentRecordId?: string;
  procedureName: string;
  additionalNote?: string | null;
  quantity: number;
  unitPriceAfn: number;
  unitPriceUsd: number;
  totalPriceAfn: number;
  totalPriceUsd: number;
  performedAt?: string;
  toothNumbers?: string | number[];
}

export interface ReceiptData {
  id: string;
  invoiceNumber: string;
  patientId: string;
  patientName: string;
  patientPhone?: string | null;
  visitId: string;
  issueDate: string;
  currency: "AFN" | "USD";
  subtotalAfn: number;
  subtotalUsd: number;
  discountAfn: number;
  discountUsd: number;
  totalAfn: number;
  totalUsd: number;
  paidAfn: number;
  paidUsd: number;
  outstandingAfn: number;
  outstandingUsd: number;
  status: "Unpaid" | "Partial" | "Paid";
  procedures: ReceiptProcedure[];
  payments: ReceiptPayment[];
  clinic: ReceiptClinic;
}

export interface AddPaymentInput {
  invoice_id: string;
  amount_afn: number;
  amount_usd: number;
  method: "Cash" | "Card" | "Mobile" | "Insurance";
  notes?: string | null;
}

export interface ExpenseCategory {
  id: string;
  name: string;
  name_ps: string;
  sort_order: number;
}

export interface Expense {
  id: string;
  category_id: string;
  description: string;
  amount_afn: number;
  amount_usd: number;
  expense_date: string;
  payment_method: string;
  notes?: string | null;
  created_at: string;
  updated_at: string;
}

export interface ExpenseListItem {
  id: string;
  category_id: string;
  category_name: string;
  category_name_ps: string;
  description: string;
  amount_afn: number;
  amount_usd: number;
  expense_date: string;
  payment_method: string;
  notes?: string | null;
  created_at: string;
  updated_at: string;
}

export interface ExpenseListParams {
  query?: string;
  categoryId?: string;
  startDate?: string;
  endDate?: string;
  page?: number;
  perPage?: number;
}

export interface ExpensePageResult {
  items: ExpenseListItem[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
  total_spent_afn: number;
  total_spent_usd: number;
}

export interface CreateExpenseInput {
  category_id: string;
  description: string;
  amount_afn: number;
  amount_usd: number;
  expense_date: string;
  payment_method: "Cash" | "Card" | "Mobile" | "Insurance";
  notes?: string | null;
}

export type UpdateExpenseInput = CreateExpenseInput;

export interface Xray {
  id: string;
  patient_id: string;
  treatment_record_id: string | null;
  file_path: string;
  is_primary: boolean;
  uploaded_at: string;
}

export interface MonthlyRevenuePoint {
  month: string;
  revenue: number;
  revenue_afn: number;
  revenue_usd: number;
  expenses: number;
  expenses_afn: number;
  expenses_usd: number;
}

export interface DailyTrendPoint {
  day: string;
  value: number;
}

export interface ReportFilter {
  filter_type: string; // "daily", "weekly", "monthly", "custom"
  start_date?: string; // "YYYY-MM-DD" for custom
  end_date?: string;   // "YYYY-MM-DD" for custom
}

export interface ReportSummary {
  active_patients: number;
  total_visits_this_month: number;
  revenue_this_month: number;
  revenue_this_month_afn: number;
  revenue_this_month_usd: number;
  expenses_this_month: number;
  expenses_this_month_afn: number;
  expenses_this_month_usd: number;
  outstanding_balance: number;
  outstanding_balance_afn: number;
  outstanding_balance_usd: number;
  completed_visits_this_month: number;
  cancelled_visits_this_month: number;
  active_patients_trend: DailyTrendPoint[];
  visits_trend: DailyTrendPoint[];
  revenue_trend: DailyTrendPoint[];
  expenses_trend: DailyTrendPoint[];
  outstanding_trend: DailyTrendPoint[];
  prev_active_patients: number;
  prev_total_visits: number;
  prev_revenue: number;
  prev_revenue_afn: number;
  prev_revenue_usd: number;
  prev_expenses: number;
  prev_expenses_afn: number;
  prev_expenses_usd: number;
  prev_outstanding: number;
  prev_outstanding_afn: number;
  prev_outstanding_usd: number;
}

export interface AppSettings {
  id: number;
  clinic_name?: string | null;
  clinic_phone?: string | null;
  clinic_address?: string | null;
  support_email?: string | null;
  clinic_logo?: string | null;
  language?: string | null;
  created_at: string;
  updated_at: string;
}

export interface UpdateSettingsInput {
  clinic_name?: string | null;
  clinic_phone?: string | null;
  clinic_address?: string | null;
  support_email?: string | null;
  clinic_logo?: string | null;
  language?: string | null;
}

export interface DashboardStats {
  daily_revenue: number;
  daily_revenue_afn: number;
  daily_revenue_usd: number;
  patients_today: number;
  outstanding_balance: number;
  outstanding_balance_afn: number;
  outstanding_balance_usd: number;
  outstanding_invoices_count: number;
  procedures_performed: number;
  yesterday_revenue: number;
  yesterday_revenue_afn: number;
  yesterday_revenue_usd: number;
  yesterday_patients: number;
  yesterday_procedures: number;
  daily_expenses: number;
  daily_expenses_afn: number;
  daily_expenses_usd: number;
  yesterday_expenses: number;
  yesterday_expenses_afn: number;
  yesterday_expenses_usd: number;
  net_today: number;
  net_today_afn: number;
  net_today_usd: number;
}

export interface PatientsFlowPoint {
  label: string;
  check_ins: number;
  visits: number;
  completed: number;
}

export interface ProcedureDistribution {
  name: string;
  count: number;
}

export interface RecentPatient {
  id: string;
  full_name: string;
  phone: string;
  age: number;
  gender: string;
  address: string;
  visit_date: string;
  status: string;
  visit_id: string;
}

export interface BackupRecord {
  id: number;
  backup_type: string;
  backup_path: string;
  cloud_provider: string;
  status: string;
  file_size: number;
  error_message: string | null;
  created_at: string;
  completed_at: string | null;
}

export interface BackupSettings {
  auto_backup_enabled: boolean;
  auto_backup_frequency: string;
  auto_backup_target: string;
  last_backup_at: string | null;
  gdrive_client_id: string | null;
  gdrive_connected: boolean;
  gdrive_folder_id: string | null;
  local_backup_enabled: boolean;
  local_backup_frequency: string;
  local_last_backup_at: string | null;
  local_next_scheduled_backup: string | null;
  gdrive_backup_enabled: boolean;
  gdrive_backup_frequency: string;
  gdrive_connected_email: string | null;
  gdrive_last_backup_at: string | null;
  gdrive_next_scheduled_backup: string | null;
  gdrive_last_sync_at: string | null;
}

export interface UpdateBackupSettingsInput {
  auto_backup_enabled?: boolean;
  auto_backup_frequency?: string;
  auto_backup_target?: string;
  gdrive_client_id?: string | null;
  local_backup_enabled?: boolean;
  local_backup_frequency?: string;
  gdrive_backup_enabled?: boolean;
  gdrive_backup_frequency?: string;
}

export interface StartAuthResult {
  auth_url: string;
}

export interface GDriveStatus {
  connected: boolean;
  email: string | null;
  last_sync_at: string | null;
}

export interface RestoreBackupInput {
  backup_id: number;
}

export interface RestoreBackupResult {
  success: boolean;
}

export interface GDriveBackupFile {
  id: string;
  name: string;
  size: number | null;
  modified_time: string | null;
}

export interface RestoreGDriveFileInput {
  file_id: string;
  file_name: string | null;
}

export interface BackupValidation {
  valid: boolean;
  file_size: number;
  table_count: number;
  error: string | null;
}

export interface ErrorResponse {
  code: string;
  message: string;
  details: string | null;
}

// Auth types
export interface AuthResponse {
  token: string;
  username: string;
  user_id: string;
}

export interface SetupResponse {
  token: string;
  username: string;
  user_id: string;
  recovery_key: string;
}

export interface HasUsersResponse {
  has_users: boolean;
}

export interface LoginInput {
  username: string;
  password: string;
}

export interface SetupUserInput {
  username: string;
  password: string;
}

export interface ChangePasswordInput {
  old_password: string;
  new_password: string;
}

export interface ResetPasswordInput {
  recovery_key: string;
  new_password: string;
}

export interface VisitListItem {
  id: string;
  patient_id: string;
  patient_name: string;
  patient_phone: string | null;
  visit_date: string;
  chief_complaint: string | null;
  status: "Open" | "Completed" | "Cancelled";
  procedures_count: number;
  total_afn: number;
  total_usd: number;
  created_at: string;
}

export interface VisitPageResult {
  items: VisitListItem[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
  open_count: number;
  completed_count: number;
  cancelled_count: number;
}

export interface VisitListParams {
  query?: string;
  status?: string;
  page?: number;
  perPage?: number;
}
