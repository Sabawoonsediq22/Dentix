-- Backfill payment rows for invoice amounts that were recorded directly on the
-- invoice (initial payment at creation) but never inserted into the payments
-- table, so payments-based revenue (dashboard/reports) matches invoice totals.
INSERT INTO payments (id, invoice_id, amount_afn, amount_usd, method, notes, received_at)
SELECT
  'PAY-' || lower(hex(randomblob(16))),
  i.id,
  max(COALESCE(i.paid_afn, 0.0) - COALESCE(p.paid_afn, 0.0), 0.0),
  max(COALESCE(i.paid_usd, 0.0) - COALESCE(p.paid_usd, 0.0), 0.0),
  'Cash',
  '',
  i.issued_at
FROM invoices i
LEFT JOIN (
  SELECT invoice_id,
         SUM(COALESCE(amount_afn, 0.0)) AS paid_afn,
         SUM(COALESCE(amount_usd, 0.0)) AS paid_usd
  FROM payments
  GROUP BY invoice_id
) p ON p.invoice_id = i.id
WHERE COALESCE(i.paid_afn, 0.0) - COALESCE(p.paid_afn, 0.0) > 0.005
   OR COALESCE(i.paid_usd, 0.0) - COALESCE(p.paid_usd, 0.0) > 0.005;
