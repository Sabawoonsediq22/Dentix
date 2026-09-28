import React, { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Modal, Button, Input, Select, DatePicker } from "../ui";
import type {
  CreateExpenseInput,
  ExpenseCategory,
  ExpenseListItem,
} from "../../types/ApiTypes";
import {
  EXPENSE_PAYMENT_METHODS,
  ExpensePaymentMethod,
  localizedCategoryName,
} from "../../types/ExpenseTypes";
import {
  validateExpenseForm,
  ExpenseFormValues,
} from "../../validation/expenseValidation";
import { format, parseISO } from "date-fns";

interface ExpenseModalProps {
  isOpen: boolean;
  onClose: () => void;
  categories: ExpenseCategory[];
  expense?: ExpenseListItem | null;
  onSave: (input: CreateExpenseInput) => void;
}

const ExpenseModal: React.FC<ExpenseModalProps> = ({
  isOpen,
  onClose,
  categories,
  expense,
  onSave,
}) => {
  const { t, i18n } = useTranslation();
  const [categoryId, setCategoryId] = useState("");
  const [description, setDescription] = useState("");
  const [amountAfn, setAmountAfn] = useState("");
  const [amountUsd, setAmountUsd] = useState("");
  const [date, setDate] = useState<Date | undefined>(new Date());
  const [method, setMethod] = useState<ExpensePaymentMethod>("Cash");
  const [notes, setNotes] = useState("");
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (!isOpen) return;
    setErrors({});
    if (expense) {
      setCategoryId(expense.category_id);
      setDescription(expense.description);
      setAmountAfn(expense.amount_afn ? String(expense.amount_afn) : "");
      setAmountUsd(expense.amount_usd ? String(expense.amount_usd) : "");
      setDate(parseISO(expense.expense_date));
      setMethod(
        (EXPENSE_PAYMENT_METHODS as readonly string[]).includes(
          expense.payment_method,
        )
          ? (expense.payment_method as ExpensePaymentMethod)
          : "Cash",
      );
      setNotes(expense.notes ?? "");
    } else {
      setCategoryId(categories[0]?.id ?? "");
      setDescription("");
      setAmountAfn("");
      setAmountUsd("");
      setDate(new Date());
      setMethod("Cash");
      setNotes("");
    }
  }, [isOpen, expense, categories]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const values: ExpenseFormValues = {
      categoryId,
      amountAfn,
      amountUsd,
      hasDate: !!date,
    };
    const result = validateExpenseForm(values, t);
    setErrors(result.errors);
    if (!result.isValid) return;

    onSave({
      category_id: categoryId,
      description: description.trim(),
      amount_afn: parseFloat(amountAfn) || 0,
      amount_usd: parseFloat(amountUsd) || 0,
      expense_date: date ? format(date, "yyyy-MM-dd") : "",
      payment_method: method,
      notes: notes.trim() || null,
    });

    onClose();
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title={
        expense
          ? t("expenses.editExpense", "Edit Expense")
          : t("expenses.newExpense", "New Expense")
      }
      size="sm"
    >
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">
            {t("expenses.form.category", "Category")}
          </label>
          <Select
            value={categoryId}
            onChange={(e) => setCategoryId(e.target.value)}
            className="w-full text-sm"
          >
            <option value="">
              {t("expenses.form.selectCategory", "Select a category")}
            </option>
            {categories.map((cat) => (
              <option key={cat.id} value={cat.id}>
                {localizedCategoryName(cat.name, cat.name_ps, i18n.language)}
              </option>
            ))}
          </Select>
          {errors.categoryId && (
            <p className="text-xs text-amber-600 mt-1">{errors.categoryId}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t("expenses.form.description", "Description")}
          </label>
          <Input
            type="text"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder={t(
              "expenses.form.descriptionPlaceholder",
              "e.g. Monthly rent payment",
            )}
            className="w-full"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t("expenses.form.date", "Date")}
          </label>
          <DatePicker
            value={date}
            onChange={(d) => setDate(d)}
            className="w-full"
          />
          {errors.expenseDate && (
            <p className="text-xs text-amber-600 mt-1">{errors.expenseDate}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t("expenses.form.amounts", "Amounts")}
          </label>
          <div className="flex gap-2">
            <div className="flex-1">
              <Input
                type="number"
                step="0.01"
                min="0"
                value={amountAfn}
                onChange={(e) => setAmountAfn(e.target.value)}
                placeholder="0.00 AFN"
                className="w-full"
              />
            </div>
            <div className="flex-1">
              <Input
                type="number"
                step="0.01"
                min="0"
                value={amountUsd}
                onChange={(e) => setAmountUsd(e.target.value)}
                placeholder="$0.00"
                className="w-full"
              />
            </div>
          </div>
          {errors.amount && (
            <p className="text-xs text-amber-600 mt-1">{errors.amount}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t("expenses.form.paymentMethod", "Payment Method")}
          </label>
          <Select
            value={method}
            onChange={(e) => setMethod(e.target.value as ExpensePaymentMethod)}
            className="w-full text-sm"
          >
            {EXPENSE_PAYMENT_METHODS.map((m) => (
              <option key={m} value={m}>
                {t(`expenses.methods.${m.toLowerCase()}`, m)}
              </option>
            ))}
          </Select>
        </div>

        <div className="flex justify-end gap-3 pt-4">
          <Button
            type="button"
            variant="outline"
            onClick={onClose}
            className="cursor-pointer"
          >
            {t("common.cancel")}
          </Button>
          <Button type="submit" className="cursor-pointer">
            {expense ? t("common.save", "Save") : t("expenses.form.add", "Add")}
          </Button>
        </div>
      </form>
    </Modal>
  );
};

export default ExpenseModal;
