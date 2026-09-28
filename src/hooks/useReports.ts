import { useQuery } from "@tanstack/react-query";
import { api } from "../lib/api";
import { AUTO_REFRESH_INTERVAL } from "./useDashboard";
import type { ReportFilter } from "../types/ApiTypes";

const DEFAULT_FILTER: ReportFilter = { filter_type: "monthly" };

export function useReportSummary(filter: ReportFilter = DEFAULT_FILTER) {
  return useQuery({
    queryKey: ["reports", "summary", filter],
    queryFn: () => api.reports.summary(filter),
    refetchInterval: AUTO_REFRESH_INTERVAL,
  });
}

export function useMonthlyRevenue(filter: ReportFilter = DEFAULT_FILTER) {
  return useQuery({
    queryKey: ["reports", "monthlyRevenue", filter],
    queryFn: () => api.reports.monthlyRevenue(filter),
  });
}