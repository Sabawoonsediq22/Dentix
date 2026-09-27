import React from "react";
import { useTranslation } from "react-i18next";
import { Button, SearchInput, Select, DatePicker } from "../ui";
import { PlusIcon, CloseIcon } from "../../shared/icons/icons";
import {
  ExpensesHeaderProps,
  localizedCategoryName,
} from "../../types/ExpenseTypes";

function formatAmount(val: number): string {
  return val.toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

const ExpensesHeader: React.FC<ExpensesHeaderProps> = ({
  totalExpenses,
  totalSpentAfn,
  totalSpentUsd,
  searchQuery,
  selectedCategoryId,
  categories,
  startDate,
  endDate,
  onSearchChange,
  onCategoryChange,
  onStartDateChange,
  onEndDateChange,
  onAddExpense,
}) => {
  const { t, i18n } = useTranslation();
  const hasDateFilter = !!startDate || !!endDate;

  return (
    <div className="space-y-4">
      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
        <SearchInput
          value={searchQuery}
          onChange={onSearchChange}
          placeholder={t(
            "expenses.searchPlaceholder",
            "Search by description, category or notes...",
          )}
          className="w-full sm:w-lg"
        />
        <Button onClick={onAddExpense} className="cursor-pointer">
          <PlusIcon className="h-4 w-4" />
          {t("expenses.addExpense", "Add Expense")}
        </Button>
      </div>

      <div className="flex flex-col xl:flex-row xl:items-center justify-between bg-gray-100 dark:bg-gray-800 rounded-lg px-4 py-3 gap-4 my-6">
        <div className="flex flex-wrap items-center gap-4">
          <div className="flex items-center gap-2 py-2">
            <span className="text-xs font-medium text-gray-500 dark:text-gray-400">
              {t("expenses.stats.totalExpenses", "Total Expenses")} |{" "}
            </span>
            <span className="text-xs font-bold text-gray-900 dark:text-white">
              {totalExpenses}
            </span>
          </div>
          <div className="flex items-center gap-2 py-2">
            <span className="text-xs font-medium text-gray-500 dark:text-gray-400">
              {t("expenses.stats.totalSpent", "Total Spent")} |{" "}
            </span>
            <span className="text-xs font-bold text-red-600 dark:text-red-400">
              {formatAmount(totalSpentAfn)} AFN
              {totalSpentUsd > 0 && ` | $${formatAmount(totalSpentUsd)}`}
            </span>
          </div>
        </div>

        <div className="flex flex-wrap items-center gap-2">
          <Select
            value={selectedCategoryId}
            onChange={(e) => onCategoryChange(e.target.value)}
            className="h-9 w-full sm:w-48 text-sm"
            aria-label={t("expenses.filters.category", "Category")}
          >
            <option value="">
              {t("expenses.filters.allCategories", "All Categories")}
            </option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>
                {localizedCategoryName(cat.name, cat.name_ps, i18n.language)}
              </option>
            ))}
          </Select>

          <DatePicker
            value={startDate}
            onChange={(date) => onStartDateChange(date)}
            placeholder={t("expenses.filters.from", "From")}
            maxDate={endDate}
            className="w-full sm:w-44"
          />
          <DatePicker
            value={endDate}
            onChange={(date) => onEndDateChange(date)}
            placeholder={t("expenses.filters.to", "To")}
            minDate={startDate}
            className="w-full sm:w-44"
          />
          {hasDateFilter && (
            <Button
              variant="ghost"
              size="sm"
              onClick={() => {
                onStartDateChange(undefined);
                onEndDateChange(undefined);
              }}
              title={t("expenses.filters.clearDates", "Clear date filters")}
              className="cursor-pointer"
            >
              <CloseIcon className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>
    </div>
  );
};

export default ExpensesHeader;
