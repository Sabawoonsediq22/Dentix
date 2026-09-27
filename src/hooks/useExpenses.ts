import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "../lib/api";
import type { CreateExpenseInput, UpdateExpenseInput } from "../types/ApiTypes";

export function useExpenses(params: {
  query?: string;
  categoryId?: string;
  startDate?: string;
  endDate?: string;
  page?: number;
  perPage?: number;
}) {
  return useQuery({
    queryKey: ["expenses", params],
    queryFn: () => api.expenses.list(params),
  });
}

export function useExpense(id: string) {
  return useQuery({
    queryKey: ["expenses", id],
    queryFn: () => api.expenses.get(id),
    enabled: !!id,
  });
}

export function useExpenseCategories() {
  return useQuery({
    queryKey: ["expenses", "categories"],
    queryFn: () => api.expenses.categories(),
    staleTime: 5 * 60 * 1000,
  });
}

function useInvalidateExpenseStats() {
  const qc = useQueryClient();
  return () => {
    qc.invalidateQueries({ queryKey: ["expenses"], refetchType: "all" });
    qc.invalidateQueries({ queryKey: ["reports"], refetchType: "all" });
    qc.invalidateQueries({ queryKey: ["dashboard"], refetchType: "all" });
  };
}

export function useCreateExpense() {
  const invalidate = useInvalidateExpenseStats();
  return useMutation({
    mutationFn: (input: CreateExpenseInput) => api.expenses.create(input),
    onSuccess: invalidate,
  });
}

export function useUpdateExpense() {
  const invalidate = useInvalidateExpenseStats();
  return useMutation({
    mutationFn: ({ id, input }: { id: string; input: UpdateExpenseInput }) =>
      api.expenses.update(id, input),
    onSuccess: invalidate,
  });
}

export function useDeleteExpense() {
  const invalidate = useInvalidateExpenseStats();
  return useMutation({
    mutationFn: (id: string) => api.expenses.delete(id),
    onSuccess: invalidate,
  });
}
