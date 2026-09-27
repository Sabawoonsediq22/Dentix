import { api } from "./api";

export type ReportFormat = "csv" | "pdf";

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
  const csv = [
    headers.join(","),
    ...rows.map((r) =>
      r.map((c) => `"${String(c).replace(/"/g, '""')}"`).join(","),
    ),
  ].join("\r\n");

  const blob = new Blob(["\uFEFF" + csv], {
    type: "text/csv;charset=utf-8;",
  });
  downloadBlob(blob, filename);
}

async function generatePDF(
  title: string,
  headers: string[],
  rows: string[][],
  filename: string,
) {
  const { default: jsPDF } = await import("jspdf");
  const { default: autoTable } = await import("jspdf-autotable");

  const doc = new jsPDF({ unit: "mm", format: "a4" });

  doc.setFontSize(16);
  doc.text(title, 14, 20);

  doc.setFontSize(9);
  doc.setTextColor(100);
  doc.text(`Generated: ${new Date().toLocaleString()}`, 14, 27);

  autoTable(doc, {
    head: [headers],
    body: rows as unknown as string[][],
    startY: 32,
    styles: { fontSize: 8 },
    headStyles: { fillColor: [0, 106, 113] },
    margin: { top: 30 },
  });

  const pageHeight = doc.internal.pageSize.getHeight();
  doc.setFontSize(8);
  doc.setTextColor(100);
  doc.text("Powered by: www.parsatechnology.com", 14, pageHeight - 8);

  doc.save(filename);
}

export async function exportPatientsReport(format: ReportFormat): Promise<void> {
  const result = await api.patients.list({ query: "", gender: "", page: 1, perPage: 10000 });
  const patients = result.items;

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

  if (format === "csv") {
    await generateCSV(headers, rows, `patients_report_${dateStamp()}.csv`);
  } else {
    await generatePDF(
      "Patient Report",
      headers,
      rows,
      `patients_report_${dateStamp()}.pdf`,
    );
  }
}

export async function exportFinancialReport(format: ReportFormat): Promise<void> {
  const result = await api.invoices.list({
    page: 1,
    perPage: 10000,
  });
  const invoices = result.items;

  const headers = [
    "Invoice #",
    "Patient Name",
    "Patient Phone",
    "Visit Date",
    "Subtotal",
    "Discount",
    "Total Amount",
    "Paid Amount",
    "Outstanding",
    "Status",
    "Issued At",
  ];

  const rows = invoices.map((inv) => [
    inv.invoice_number,
    inv.patient_name,
    inv.patient_phone ?? "",
    formatDate(inv.visit_date),
    inv.subtotal_afn.toFixed(2),
    inv.subtotal_usd.toFixed(2),
    inv.discount_afn.toFixed(2),
    inv.discount_usd.toFixed(2),
    inv.total_afn.toFixed(2),
    inv.total_usd.toFixed(2),
    inv.paid_afn.toFixed(2),
    inv.paid_usd.toFixed(2),
    inv.outstanding_afn.toFixed(2),
    inv.outstanding_usd.toFixed(2),
    inv.status,
    formatDate(inv.issued_at),
  ]);

  if (format === "csv") {
    await generateCSV(headers, rows, `financial_report_${dateStamp()}.csv`);
  } else {
    await generatePDF(
      "Financial Report",
      headers,
      rows,
      `financial_report_${dateStamp()}.pdf`,
    );
  }
}

export async function exportTreatmentReport(format: ReportFormat): Promise<void> {
  const procedures = await api.procedures.list();

  const headers = [
    "Procedure Name",
    "Price (AFN)",
    "Additional Note",
    "Created At",
  ];

  const rows = procedures.map((p) => [
    p.name,
    `AFN: ${p.procedure_price_afn.toFixed(2)} / $: ${p.procedure_price_usd.toFixed(2)}`,
    p.additional_note ?? "",
    formatDate(p.created_at),
  ]);

  if (format === "csv") {
    await generateCSV(headers, rows, `treatment_report_${dateStamp()}.csv`);
  } else {
    await generatePDF(
      "Treatment Report",
      headers,
      rows,
      `treatment_report_${dateStamp()}.pdf`,
    );
  }
}

export async function exportExpensesReport(format: ReportFormat): Promise<void> {
  const result = await api.expenses.list({ page: 1, perPage: 10000 });
  const expenses = result.items;

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

  if (format === "csv") {
    await generateCSV(headers, rows, `expenses_report_${dateStamp()}.csv`);
  } else {
    await generatePDF(
      "Expense Report",
      headers,
      rows,
      `expenses_report_${dateStamp()}.pdf`,
    );
  }
}
