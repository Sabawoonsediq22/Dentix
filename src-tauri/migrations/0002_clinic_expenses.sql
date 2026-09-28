CREATE TABLE IF NOT EXISTS expense_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    name_ps TEXT NOT NULL DEFAULT '',
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS expenses (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES expense_categories(id),
    description TEXT,
    amount_afn REAL NOT NULL DEFAULT 0 CHECK(amount_afn >= 0),
    amount_usd REAL NOT NULL DEFAULT 0 CHECK(amount_usd >= 0),
    expense_date TEXT NOT NULL,
    payment_method TEXT NOT NULL DEFAULT 'Cash',
    notes TEXT DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-rent', 'Rent', 'کرایه', 1);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-salaries', 'Salaries', 'معاشات', 2);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-utilities', 'Utilities', 'آب و برق', 3);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-supplies', 'Medical Supplies', 'لوازم طبی', 4);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-equipment', 'Equipment', 'تجهیزات', 5);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-maintenance', 'Maintenance', 'ترمیم و نگه‌داری', 6);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-transport', 'Transport', 'نقلیه', 7);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-marketing', 'Marketing', 'تبلیغات', 8);
INSERT OR IGNORE INTO expense_categories (id, name, name_ps, sort_order) VALUES ('CAT-other', 'Other', 'نور', 9);

CREATE INDEX IF NOT EXISTS idx_expenses_expense_date ON expenses(expense_date);
CREATE INDEX IF NOT EXISTS idx_expenses_category ON expenses(category_id);
CREATE INDEX IF NOT EXISTS idx_expenses_created_at ON expenses(created_at);

ANALYZE;
