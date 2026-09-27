import type { ExpenseCategory, ExpenseListItem } from "./ApiTypes";

export interface ExpensesHeaderProps {
  totalExpenses: number;
  totalSpentAfn: number;
  totalSpentUsd: number;
  searchQuery: string;
  selectedCategoryId: string;
  categories: ExpenseCategory[];
  startDate?: Date;
  endDate?: Date;
  onSearchChange: (query: string) => void;
  onCategoryChange: (categoryId: string) => void;
  onStartDateChange: (date?: Date) => void;
  onEndDateChange: (date?: Date) => void;
  onAddExpense: () => void;
}

export interface ExpensesTableProps {
  expenses: ExpenseListItem[];
  onEdit?: (expense: ExpenseListItem) => void;
  onDelete?: (expense: ExpenseListItem) => void;
}

export const EXPENSE_PAYMENT_METHODS = [
  "Cash",
  "Card",
  "Mobile",
  "Insurance",
] as const;

export type ExpensePaymentMethod = (typeof EXPENSE_PAYMENT_METHODS)[number];

export function localizedCategoryName(
  name: string,
  namePs: string,
  language: string,
): string {
  return language === "ps" && namePs ? namePs : name;
}
