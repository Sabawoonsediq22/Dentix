import React from "react";
import { useTranslation } from "react-i18next";
import { ExpensesTableProps } from "../../types/ExpenseTypes";
import { localizedCategoryName } from "../../types/ExpenseTypes";
import { Badge } from "../ui/Badge";
import { EditIcon, TrashIcon } from "../../shared/icons/icons";

function formatDate(date: string | null | undefined): string {
  if (!date) return "-";
  try {
    const match = date.match(/^(\d{4})-(\d{2})-(\d{2})/);
    const d = match
      ? new Date(Number(match[1]), Number(match[2]) - 1, Number(match[3]))
      : new Date(date);
    return d.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "2-digit",
    });
  } catch {
    return date;
  }
}

function formatCurrency(val: number): string {
  return val.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

function formatDualCurrency(afn: number, usd: number): string {
  const parts: string[] = [];
  if (afn > 0) parts.push(`${formatCurrency(afn)} AFN`);
  if (usd > 0) parts.push(`$${formatCurrency(usd)}`);
  return parts.length > 0 ? parts.join(" + ") : `${formatCurrency(0)} AFN`;
}

const ExpensesTable: React.FC<ExpensesTableProps> = ({
  expenses,
  onEdit,
  onDelete,
}) => {
  const { t, i18n } = useTranslation();

  const categoryLabel = (expense: (typeof expenses)[number]) =>
    localizedCategoryName(
      expense.category_name,
      expense.category_name_ps,
      i18n.language,
    );

  const methodLabel = (method: string) =>
    t(`expenses.methods.${method.toLowerCase()}`, method);

  if (expenses.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-16 bg-white dark:bg-gray-800">
        <p className="text-gray-500 dark:text-gray-400 text-sm">
          {t(
            "expenses.table.empty",
            "No expenses found matching your criteria.",
          )}
        </p>
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-gray-800">
      <div className="hidden md:block overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b bg-gray-100 dark:bg-gray-900 border-gray-200 dark:border-gray-700">
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400 whitespace-nowrap">
                {t("expenses.table.no", "NO.")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.date", "DATE")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.category", "CATEGORY")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.description", "DESCRIPTION")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.amount", "AMOUNT")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.method", "METHOD")}
              </th>
              <th className="py-3 px-4 font-bold text-xs uppercase tracking-wider text-gray-800 dark:text-gray-400">
                {t("expenses.table.actions", "ACTIONS")}
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100 dark:divide-gray-700/60">
            {expenses.map((expense, index) => (
              <tr
                key={`${expense.id}-${index}`}
                className="hover:bg-gray-50 dark:hover:bg-gray-700/40 transition-colors"
              >
                <td className="py-3 px-4 text-sm font-medium text-gray-600 dark:text-gray-300 whitespace-nowrap">
                  {index + 1}
                </td>
                <td className="py-3 px-4 text-sm text-gray-600 dark:text-gray-300 whitespace-nowrap">
                  {formatDate(expense.expense_date)}
                </td>
                <td className="py-3 px-4">
                  <Badge variant="secondary" className="text-xs font-medium">
                    {categoryLabel(expense)}
                  </Badge>
                </td>
                <td className="py-3 px-4 text-sm text-gray-600 dark:text-gray-300">
                  <div className="max-w-64 truncate" title={expense.description}>
                    {expense.description}
                  </div>
                </td>
                <td className="py-3 px-4 text-sm font-medium text-red-600 dark:text-red-400 whitespace-nowrap">
                  {formatDualCurrency(expense.amount_afn, expense.amount_usd)}
                </td>
                <td className="py-3 px-4 text-sm text-gray-600 dark:text-gray-300 whitespace-nowrap">
                  {methodLabel(expense.payment_method)}
                </td>
                <td className="py-3 px-4">
                  <div className="flex items-center gap-2">
                    <button
                      type="button"
                      onClick={() => onEdit?.(expense)}
                      title={t("expenses.actions.edit", "Edit")}
                      className="p-1.5 rounded-md text-gray-500 hover:text-blue-600 hover:bg-blue-50 dark:text-gray-400 dark:hover:text-blue-400 dark:hover:bg-blue-900/20 transition-colors cursor-pointer"
                    >
                      <EditIcon className="h-5 w-5" />
                    </button>
                    <button
                      type="button"
                      onClick={() => onDelete?.(expense)}
                      title={t("expenses.actions.delete", "Delete")}
                      className="p-1.5 rounded-md text-gray-500 hover:text-red-600 hover:bg-red-50 dark:text-gray-400 dark:hover:text-red-400 dark:hover:bg-red-900/20 transition-colors cursor-pointer"
                    >
                      <TrashIcon className="h-5 w-5" />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="md:hidden divide-y divide-gray-100 dark:divide-gray-700/60">
        {expenses.map((expense, idx) => (
          <div
            key={`${expense.id}-${idx}`}
            className="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/40 transition-colors"
          >
            <div className="flex items-center justify-between gap-3">
              <div className="flex-1 min-w-0">
                <p
                  className="text-sm font-semibold text-gray-900 dark:text-white truncate"
                  title={expense.description}
                >
                  {expense.description}
                </p>
                <p className="text-xs text-gray-500 dark:text-gray-400 truncate">
                  {categoryLabel(expense)}
                </p>
              </div>
              <span className="text-sm font-bold text-red-600 dark:text-red-400 whitespace-nowrap">
                {formatDualCurrency(expense.amount_afn, expense.amount_usd)}
              </span>
            </div>
            <div className="mt-2 grid grid-cols-2 gap-x-4 gap-y-1 text-xs text-gray-500 dark:text-gray-400">
              <span>
                {t("expenses.table.dateLabel", "Date:")}{" "}
                {formatDate(expense.expense_date)}
              </span>
              <span>
                {t("expenses.table.methodLabel", "Method:")}{" "}
                {methodLabel(expense.payment_method)}
              </span>
            </div>
            <div className="mt-3 flex items-center justify-end gap-2">
              <button
                type="button"
                onClick={() => onEdit?.(expense)}
                title={t("expenses.actions.edit", "Edit")}
                className="p-1.5 rounded-md text-gray-500 hover:text-blue-600 hover:bg-blue-50 dark:text-gray-400 dark:hover:text-blue-400 dark:hover:bg-blue-900/20 transition-colors cursor-pointer"
              >
                <EditIcon className="h-5 w-5" />
              </button>
              <button
                type="button"
                onClick={() => onDelete?.(expense)}
                title={t("expenses.actions.delete", "Delete")}
                className="p-1.5 rounded-md text-gray-500 hover:text-red-600 hover:bg-red-50 dark:text-gray-400 dark:hover:text-red-400 dark:hover:bg-red-900/20 transition-colors cursor-pointer"
              >
                <TrashIcon className="h-5 w-5" />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ExpensesTable;
