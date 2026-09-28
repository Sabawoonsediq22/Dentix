import { useQuery } from "@tanstack/react-query";
import type { RecentPatient } from "../types/ApiTypes";
import { api } from "../lib/api";

export const AUTO_REFRESH_INTERVAL = 300000;

export function useDashboardStats() {
  return useQuery({
    queryKey: ["dashboard", "stats"],
    queryFn: () => api.dashboard.stats(),
    refetchInterval: AUTO_REFRESH_INTERVAL,
  });
}

export function usePatientsFlow(mode: "daily" | "weekly" | "monthly" = "daily") {
  return useQuery({
    queryKey: ["dashboard", "patientsFlow", mode],
    queryFn: () => api.dashboard.patientsFlow(mode),
    refetchInterval: AUTO_REFRESH_INTERVAL,
  });
}

export function useProcedureDistribution(mode: "daily" | "weekly" | "monthly" = "daily") {
  return useQuery({
    queryKey: ["dashboard", "procedureDistribution", mode],
    queryFn: () => api.dashboard.procedureDistribution(mode),
    refetchInterval: AUTO_REFRESH_INTERVAL,
  });
}

export function useRecentPatients(limit?: number) {
  return useQuery({
    queryKey: ["dashboard", "recentPatients", limit],
    queryFn: () => api.dashboard.recentPatients(),
    refetchInterval: AUTO_REFRESH_INTERVAL,
    select: limit
      ? (data: RecentPatient[] | undefined) => data?.slice(0, limit)
      : undefined,
  });
}
