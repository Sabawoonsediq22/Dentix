import { api } from "./api";
import type { ReportFilter } from "../types/ApiTypes";

export type ReportFormat = "csv" | "pdf";

interface ReportRange {
  start: string;
  end: string;
}

const toISODate = (date: Date) =>
  `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(
    date.getDate(),
  ).padStart(2, "0")}`;

function resolveRange(filter?: ReportFilter): ReportRange | null {
  if (!filter) return null;

  const now = new Date();

  switch (filter.filter_type) {
    case "daily": {
      const today = toISODate(now);
      return { start: today, end: today };
    }
    case "weekly": {
      const start = new Date(now);
      start.setDate(start.getDate() - 6);
      return { start: toISODate(start), end: toISODate(now) };
    }
    case "custom": {
      const start = filter.start_date ?? filter.end_date;
      const end = filter.end_date ?? filter.start_date;
      if (!start || !end) return null;
      return { start, end };
    }
    case "monthly": {
      const monthStart = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(
        2,
        "0",
      )}-01`;
      return { start: monthStart, end: toISODate(now) };
    }
    default:
      return null;
  }
}

function toDay(value: string): string {
  if (value.length > 10) {
    const parsed = new Date(value);
    if (!Number.isNaN(parsed.getTime())) return toISODate(parsed);
  }
  return value.slice(0, 10);
}

function inRange(value: string | null | undefined, range: ReportRange | null): boolean {
  if (!range) return true;
  if (!value) return false;
  const day = toDay(value);
  return day >= range.start && day <= range.end;
}

function periodLabel(range: ReportRange | null): string | null {
  if (!range) return null;
  return range.start === range.end ? range.start : `${range.start} to ${range.end}`;
}

function fileStamp(range: ReportRange | null): string {
  const label = periodLabel(range);
  return label ? label.replace(/ to /g, "_") : dateStamp();
}

export function describeReportPeriod(filter?: ReportFilter): string {
  const range = resolveRange(filter);
  return periodLabel(range) ?? "All time";
}

const DEFAULT_CLINIC_NAME = "Dental Clinic";
const DEFAULT_CLINIC_ADDRESS = "Jalalabad,Nangarhar";

async function getClinicHeader(): Promise<{ name: string; address: string }> {
  try {
    const settings = await api.settings.get();
    return {
      name: settings?.clinic_name?.trim() || DEFAULT_CLINIC_NAME,
      address: settings?.clinic_address?.trim() || DEFAULT_CLINIC_ADDRESS,
    };
  } catch {
    return { name: DEFAULT_CLINIC_NAME, address: DEFAULT_CLINIC_ADDRESS };
  }
}

function downloadBlob(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

function formatDate(dateStr: string) {
  if (!dateStr) return "";
  try {
    return new Date(dateStr).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  } catch {
    return dateStr;
  }
}

function dateStamp() {
  return new Date().toISOString().slice(0, 10);
}

async function generateCSV(
  headers: string[],
  rows: string[][],
  filename: string,
) {
  const quote = (value: string) => `"${String(value).replace(/"/g, '""')}"`;
  const clinic = await getClinicHeader();

  const csv = [
    quote(clinic.name),
    quote(clinic.address),
    "",
    headers.join(","),
    ...rows.map((r) => r.map(quote).join(",")),
  ].join("\r\n");

  const blob = new Blob(["\uFEFF" + csv], {
    type: "text/csv;charset=utf-8;",
  });
  downloadBlob(blob, filename);
}

interface PdfOptions {
  period?: string | null;
  foot?: string[];
  rightAlignFrom?: number;
}

async function generatePDF(
  title: string,
  headers: string[],
  rows: string[][],
  filename: string,
  options: PdfOptions = {},
) {
  const { default: jsPDF } = await import("jspdf");
  const { default: autoTable } = await import("jspdf-autotable");

  const clinic = await getClinicHeader();
  const doc = new jsPDF({ unit: "mm", format: "a4" });

  const MAX_WIDTH = 182;
  let y = 14;
  const pageWidth = doc.internal.pageSize.getWidth();
  const centerX = pageWidth / 2;

  const write = (
    text: string,
    size: number,
    color: [number, number, number],
    style: { bold?: boolean; lineHeight?: number; align?: "left" | "center" } = {},
  ) => {
    if (!text) return;
    doc.setFont("helvetica", style.bold ? "bold" : "normal");
    doc.setFontSize(size);
    doc.setTextColor(color[0], color[1], color[2]);
    const centered = style.align === "center";
    const x = centered ? centerX : 14;
    const lines = doc.splitTextToSize(text, MAX_WIDTH) as string[];
    for (const line of lines) {
      doc.text(line, x, y, centered ? { align: "center" } : undefined);
      y += style.lineHeight ?? 4.5;
    }
  };

  write(clinic.name, 14, [0, 106, 113], { bold: true, lineHeight: 5.5, align: "center" });
  write(clinic.address, 9, [100, 100, 100], { align: "center" });
  y += 2;
  write(title, 16, [0, 0, 0], { bold: true, lineHeight: 6 });
  write(`Generated: ${new Date().toLocaleString()}`, 9, [100, 100, 100]);
  if (options.period) write(`Period: ${options.period}`, 9, [100, 100, 100]);

  const startY = y + 1.5;

  const columnStyles: Record<number, { halign: "right" }> = {};
  if (options.rightAlignFrom !== undefined) {
    for (let i = options.rightAlignFrom; i < headers.length; i++) {
      columnStyles[i] = { halign: "right" };
    }
  }

  autoTable(doc, {
    head: [headers],
    body: rows as unknown as string[][],
    foot: options.foot ? [options.foot] : undefined,
    startY,
    styles: { fontSize: 8, cellPadding: 2.5 },
    headStyles: { fillColor: [0, 106, 113], textColor: [255, 255, 255], fontStyle: "bold" },
    footStyles: { fillColor: [0, 106, 113], textColor: [255, 255, 255], fontStyle: "bold" },
    alternateRowStyles: { fillColor: [240, 248, 248] },
    columnStyles: options.rightAlignFrom !== undefined ? columnStyles : undefined,
    margin: { top: startY },
  });

  const pageHeight = doc.internal.pageSize.getHeight();
  doc.setFont("helvetica", "normal");
  doc.setFontSize(8);
  doc.setTextColor(100);
  doc.text("Powered by: www.parsatechnology.com", 14, pageHeight - 8);

  doc.save(filename);
}

function formatPeriodKey(key: string, monthly: boolean): string {
  const [year, month, day] = key.split("-").map(Number);
  if (!year || !month) return key;
  const date = new Date(year, month - 1, day || 1);
  return date.toLocaleDateString(
    "en-US",
    monthly ? { month: "short", year: "numeric" } : { month: "short", day: "numeric", year: "numeric" },
  );
}

export async function exportPatientsReport(
  format: ReportFormat,
  filter?: ReportFilter,
): Promise<void> {
  const range = resolveRange(filter);
  const period = periodLabel(range);

  const [result, visitsResult] = await Promise.all([
    api.patients.list({ query: "", gender: "All", page: 1, perPage: 10000 }),
    range ? api.visits.listAll({ page: 1, perPage: 10000 }) : Promise.resolve(null),
  ]);

  const activeInPeriod = new Set(
    (visitsResult?.items ?? [])
      .filter((v) => inRange(v.visit_date, range))
      .map((v) => v.patient_id),
  );

  const patients = result.items.filter(
    (p) => !range || activeInPeriod.has(p.id) || inRange(p.created_at, range),
  );

  const headers = [
    "Full Name",
    "Phone",
    "Age",
    "Gender",
    "Address",
    "Last Visit",
    "Created At",
  ];

  const rows = patients.map((p) => [
    p.full_name,
    p.phone,
    String(p.age),
    p.gender,
    p.address ?? "",
    p.last_visit ? formatDate(p.last_visit) : "",
    formatDate(p.created_at),
  ]);

  const stamp = fileStamp(range);

  if (format === "csv") {
    await generateCSV(headers, rows, `patients_report_${stamp}.csv`);
  } else {
    await generatePDF(
      "Patient Report",
      headers,
      rows,
      `patients_report_${stamp}.pdf`,
      { period },
    );
  }
}

export async function exportFinancialReport(
  format: ReportFormat,
  filter?: ReportFilter,
): Promise<void> {
  const range = resolveRange(filter);
  const period = periodLabel(range);

  const queryFilter: ReportFilter = range
    ? { filter_type: "custom", start_date: range.start, end_date: range.end }
    : { filter_type: filter?.filter_type ?? "monthly" };

  const points = await api.reports.monthlyRevenue(queryFilter);

  const spanDays = range
    ? Math.round((Date.parse(range.end) - Date.parse(range.start)) / 86_400_000) + 1
    : Number.POSITIVE_INFINITY;
  const groupByMonth = spanDays > 45;

  interface Bucket {
    key: string;
    revenueAfn: number;
    revenueUsd: number;
    expensesAfn: number;
    expensesUsd: number;
  }

  const buckets = new Map<string, Bucket>();
  for (const p of points) {
    const key = (groupByMonth ? p.month.slice(0, 7) : p.month.slice(0, 10)) || p.month;
    const bucket = buckets.get(key) ?? {
      key,
      revenueAfn: 0,
      revenueUsd: 0,
      expensesAfn: 0,
      expensesUsd: 0,
    };
    bucket.revenueAfn += p.revenue_afn;
    bucket.revenueUsd += p.revenue_usd;
    bucket.expensesAfn += p.expenses_afn;
    bucket.expensesUsd += p.expenses_usd;
    buckets.set(key, bucket);
  }

  const active = [...buckets.values()].filter(
    (b) => b.revenueAfn || b.revenueUsd || b.expensesAfn || b.expensesUsd,
  );

  const money = (value: number) =>
    value.toLocaleString("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 });

  const headers = [
    "Period",
    "Revenue (AFN)",
    "Revenue (USD)",
    "Expenses (AFN)",
    "Expenses (USD)",
    "Profit (AFN)",
    "Profit (USD)",
  ];

  const toRow = (label: string, b: Bucket) => [
    label,
    money(b.revenueAfn),
    money(b.revenueUsd),
    money(b.expensesAfn),
    money(b.expensesUsd),
    money(b.revenueAfn - b.expensesAfn),
    money(b.revenueUsd - b.expensesUsd),
  ];

  const rows = active.map((b) => toRow(formatPeriodKey(b.key, groupByMonth), b));

  const totals = active.reduce<Bucket>(
    (acc, b) => ({
      key: "TOTAL",
      revenueAfn: acc.revenueAfn + b.revenueAfn,
      revenueUsd: acc.revenueUsd + b.revenueUsd,
      expensesAfn: acc.expensesAfn + b.expensesAfn,
      expensesUsd: acc.expensesUsd + b.expensesUsd,
    }),
    { key: "TOTAL", revenueAfn: 0, revenueUsd: 0, expensesAfn: 0, expensesUsd: 0 },
  );
  const totalRow = toRow("TOTAL", totals);

  const stamp = fileStamp(range);

  if (format === "csv") {
    await generateCSV(headers, [...rows, totalRow], `financial_report_${stamp}.csv`);
  } else {
    await generatePDF(
      "Financial Report",
      headers,
      rows,
      `financial_report_${stamp}.pdf`,
      { period, foot: totalRow, rightAlignFrom: 1 },
    );
  }
}

export async function exportTreatmentReport(
  format: ReportFormat,
  filter?: ReportFilter,
): Promise<void> {
  const range = resolveRange(filter);
  const period = periodLabel(range);

  const visitsResult = await api.visits.listAll({ page: 1, perPage: 10000 });
  const visits = visitsResult.items
    .filter((v) => inRange(v.visit_date, range))
    .sort((a, b) => a.visit_date.localeCompare(b.visit_date));

  const patientIds = Array.from(new Set(visits.map((v) => v.patient_id)));
  const histories = await Promise.all(
    patientIds.map((patientId) => api.visits.getWithTreatments(patientId)),
  );

  const patientNames = new Map(visits.map((v) => [v.patient_id, v.patient_name]));
  const visitDates = new Map(visits.map((v) => [v.id, v.visit_date]));

  const headers = [
    "Patient Name",
    "Visit Date",
    "Procedure",
    "Quantity",
    "Unit Price (AFN)",
    "Unit Price (USD)",
    "Total (AFN)",
    "Total (USD)",
    "Performed At",
  ];

  const rows: string[][] = [];
  for (let i = 0; i < histories.length; i++) {
    const patientName = patientNames.get(patientIds[i]) ?? "";
    for (const visit of histories[i]) {
      const visitDate = visitDates.get(visit.visit_id);
      if (!visitDate) continue;
      for (const proc of visit.procedures) {
        rows.push([
          patientName,
          formatDate(visitDate),
          proc.procedure_name,
          String(proc.number_of_procedures),
          proc.unit_price_afn.toFixed(2),
          proc.unit_price_usd.toFixed(2),
          proc.total_price_afn.toFixed(2),
          proc.total_price_usd.toFixed(2),
          proc.performed_at ? formatDate(proc.performed_at) : "",
        ]);
      }
    }
  }

  const stamp = fileStamp(range);

  if (format === "csv") {
    await generateCSV(headers, rows, `treatment_report_${stamp}.csv`);
  } else {
    await generatePDF(
      "Treatment Report",
      headers,
      rows,
      `treatment_report_${stamp}.pdf`,
      { period },
    );
  }
}

export async function exportExpensesReport(
  format: ReportFormat,
  filter?: ReportFilter,
): Promise<void> {
  const range = resolveRange(filter);
  const period = periodLabel(range);

  const result = await api.expenses.list({
    page: 1,
    perPage: 10000,
    startDate: range?.start,
    endDate: range?.end,
  });
  const expenses = result.items.filter((e) => inRange(e.expense_date, range));

  const headers = [
    "Date",
    "Category",
    "Description",
    "Amount (AFN)",
    "Amount (USD)",
    "Payment Method",
    "Notes",
    "Created At",
  ];

  const rows = expenses.map((e) => [
    e.expense_date,
    e.category_name,
    e.description,
    e.amount_afn.toFixed(2),
    e.amount_usd.toFixed(2),
    e.payment_method,
    e.notes ?? "",
    formatDate(e.created_at),
  ]);

  const stamp = fileStamp(range);

  if (format === "csv") {
    await generateCSV(headers, rows, `expenses_report_${stamp}.csv`);
  } else {
    await generatePDF(
      "Expense Report",
      headers,
      rows,
      `expenses_report_${stamp}.pdf`,
      { period },
    );
  }
}
