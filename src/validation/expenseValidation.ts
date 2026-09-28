import type { TFunction } from "i18next";

export interface ValidationResult {
  isValid: boolean;
  errors: Record<string, string>;
}

export interface ExpenseFormValues {
  categoryId: string;
  amountAfn: string;
  amountUsd: string;
  hasDate: boolean;
}

export const validateExpenseForm = (
  values: ExpenseFormValues,
  t: TFunction,
): ValidationResult => {
  const errors: Record<string, string> = {};

  if (!values.categoryId) {
    errors.categoryId = t(
      "expenses.errors.categoryRequired",
      "Please select a category",
    );
  }

  if (!values.hasDate) {
    errors.expenseDate = t(
      "expenses.errors.dateRequired",
      "Expense date is required",
    );
  }

  const afn = parseFloat(values.amountAfn) || 0;
  const usd = parseFloat(values.amountUsd) || 0;

  if (afn < 0 || usd < 0) {
    errors.amount = t(
      "expenses.errors.negativeAmount",
      "Amounts cannot be negative",
    );
  } else if (afn === 0 && usd === 0) {
    errors.amount = t(
      "expenses.errors.amountRequired",
      "Enter an amount in AFN or USD",
    );
  }

  return { isValid: Object.keys(errors).length === 0, errors };
};
