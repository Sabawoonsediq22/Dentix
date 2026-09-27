import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { LoadingSpinner, Pagination, ConfirmDialog } from "../components/ui";
import ExpensesHeader from "../components/expenses/ExpensesHeader";
import ExpensesTable from "../components/expenses/ExpensesTable";
import ExpenseModal from "../components/expenses/ExpenseModal";
import {
  useExpenses,
  useExpenseCategories,
  useCreateExpense,
  useUpdateExpense,
  useDeleteExpense,
} from "../hooks/useExpenses";
import { useDebounce } from "../hooks/useDebounce";
import { toast } from "sonner";
import { format } from "date-fns";
import type {
  CreateExpenseInput,
  ExpenseListItem,
} from "../types/ApiTypes";

const PAGE_SIZE = 10;
const SEARCH_DEBOUNCE_MS = 300;

const Expenses: React.FC = () => {
  const { t } = useTranslation();

  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategoryId, setSelectedCategoryId] = useState("");
  const [startDate, setStartDate] = useState<Date | undefined>(undefined);
  const [endDate, setEndDate] = useState<Date | undefined>(undefined);
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage, setItemsPerPage] = useState(PAGE_SIZE);

  const [showExpenseModal, setShowExpenseModal] = useState(false);
  const [selectedExpense, setSelectedExpense] =
    useState<ExpenseListItem | null>(null);
  const [expenseToDelete, setExpenseToDelete] =
    useState<ExpenseListItem | null>(null);

  const debouncedSearchQuery = useDebounce(searchQuery, SEARCH_DEBOUNCE_MS);

  useEffect(() => {
    setCurrentPage(1);
  }, [debouncedSearchQuery, selectedCategoryId, startDate, endDate]);

  const { data, isLoading, error } = useExpenses({
    query: debouncedSearchQuery || undefined,
    categoryId: selectedCategoryId || undefined,
    startDate: startDate ? format(startDate, "yyyy-MM-dd") : undefined,
    endDate: endDate ? format(endDate, "yyyy-MM-dd") : undefined,
    page: currentPage,
    perPage: itemsPerPage,
  });

  const { data: categories = [] } = useExpenseCategories();
  const createMutation = useCreateExpense();
  const updateMutation = useUpdateExpense();
  const deleteMutation = useDeleteExpense();

  const handleSearchChange = (query: string) => setSearchQuery(query);

  const handleAddExpense = () => {
    setSelectedExpense(null);
    setShowExpenseModal(true);
  };

  const handleEditExpense = (expense: ExpenseListItem) => {
    setSelectedExpense(expense);
    setShowExpenseModal(true);
  };

  const handleSave = (input: CreateExpenseInput) => {
    if (selectedExpense) {
      updateMutation.mutate(
        { id: selectedExpense.id, input },
        {
          onSuccess: () => {
            toast.success(
              t(
                "expenses.notifications.updated",
                "Expense updated successfully",
              ),
            );
          },
          onError: (err) => {
            toast.error(
              `${t("expenses.notifications.updateError", "Failed to update expense")}: ${String(err)}`,
            );
          },
        },
      );
    } else {
      createMutation.mutate(input, {
        onSuccess: () => {
          toast.success(
            t(
              "expenses.notifications.created",
              "Expense created successfully",
            ),
          );
        },
        onError: (err) => {
          toast.error(
            `${t("expenses.notifications.createError", "Failed to create expense")}: ${String(err)}`,
          );
        },
      });
    }
  };

  const handleDeleteConfirm = () => {
    if (!expenseToDelete) return;
    deleteMutation.mutate(expenseToDelete.id, {
      onSuccess: () => {
        toast.success(
          t("expenses.notifications.deleted", "Expense deleted successfully"),
        );
        setExpenseToDelete(null);
      },
      onError: (err) => {
        toast.error(
          `${t("expenses.notifications.deleteError", "Failed to delete expense")}: ${String(err)}`,
        );
      },
    });
  };

  const handlePageChange = (page: number) => {
    if (page >= 1 && page <= (data?.total_pages ?? 1)) {
      setCurrentPage(page);
    }
  };

  const handleItemsPerPageChange = (items: number) => {
    setItemsPerPage(items);
    setCurrentPage(1);
  };

  return (
    <div className="flex flex-col h-full">
      <ExpensesHeader
        totalExpenses={data?.total ?? 0}
        totalSpentAfn={data?.total_spent_afn ?? 0}
        totalSpentUsd={data?.total_spent_usd ?? 0}
        searchQuery={searchQuery}
        selectedCategoryId={selectedCategoryId}
        categories={categories}
        startDate={startDate}
        endDate={endDate}
        onSearchChange={handleSearchChange}
        onCategoryChange={setSelectedCategoryId}
        onStartDateChange={setStartDate}
        onEndDateChange={setEndDate}
        onAddExpense={handleAddExpense}
      />

      <div className="flex-1 overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700 mb-3">
        {isLoading ? (
          <div className="flex h-full items-center justify-center">
            <LoadingSpinner
              size="lg"
              text={t("expenses.loading", "Loading expenses...")}
            />
          </div>
        ) : error ? (
          <div className="flex h-full items-center justify-center">
            <div className="text-lg text-red-500">
              {t("expenses.errorLoading", "Error loading expenses")}:{" "}
              {String(error)}
            </div>
          </div>
        ) : (
          <ExpensesTable
            expenses={data?.items ?? []}
            onEdit={handleEditExpense}
            onDelete={setExpenseToDelete}
          />
        )}
      </div>

      <Pagination
        currentPage={currentPage}
        totalPages={data?.total_pages ?? 1}
        totalItems={data?.total ?? 0}
        itemsPerPage={itemsPerPage}
        onPageChange={handlePageChange}
        onItemsPerPageChange={handleItemsPerPageChange}
      />

      <ExpenseModal
        isOpen={showExpenseModal}
        onClose={() => setShowExpenseModal(false)}
        categories={categories}
        expense={selectedExpense}
        onSave={handleSave}
      />

      <ConfirmDialog
        isOpen={!!expenseToDelete}
        onClose={() => setExpenseToDelete(null)}
        onConfirm={handleDeleteConfirm}
        title={t("expenses.deleteConfirmTitle", "Delete expense?")}
        description={t(
          "expenses.deleteConfirmDescription",
          "This action cannot be undone.",
        )}
        confirmText={t("common.delete", "Delete")}
        confirmVariant="destructive"
        isLoading={deleteMutation.isPending}
      >
        <p className="text-sm text-muted-foreground">
          {expenseToDelete?.description}
        </p>
      </ConfirmDialog>
    </div>
  );
};

export default Expenses;
