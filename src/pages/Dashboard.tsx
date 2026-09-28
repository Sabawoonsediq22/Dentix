import React, { useState, useEffect, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { format } from "date-fns";
import { useDashboardStats, usePatientsFlow, useRecentPatients } from "../hooks/useDashboard";
import { useReportSummary } from "../hooks/useReports";
import { useUpdateVisitStatus } from "../hooks/useVisits";
import { toast } from "../lib/toast-utils";
import StatCard, { type CardAccent } from "../components/dashboard/StatCard";
import RecentPatientsTable from "../components/dashboard/RecentPatientsTable";
import { ActivityIcon, PlusIcon } from "../shared/icons/icons";
import { Badge, Button, Select } from "../components/ui";
import ChartCard from "../components/dashboard/ChartCard";
import Chart from "react-apexcharts";
import type { ApexOptions } from "apexcharts";
import type { ReportFilter } from "../types/ApiTypes";

const FINANCE_COLORS = {
  revenue: "#10b981",
  expenses: "#ef4444",
  outstanding: "#f59e0b",
};

const formatAFN = (val: number) =>
  val.toLocaleString("en-US", {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }) + " AFN";

const formatUSD = (val: number) =>
  val === 0
    ? ""
    : "$" + val.toLocaleString("en-US", {
        minimumFractionDigits: 0,
        maximumFractionDigits: 2,
      });

const computeTrend = (
  today: number,
  yesterday: number,
): { value: string; positive: boolean } | undefined => {
  if (yesterday <= 0) return undefined;
  const rawPct = ((today - yesterday) / yesterday) * 100;
  const clamped = Math.max(-100, rawPct);
  const sign = clamped >= 0 ? "+" : "";
  return { value: `${sign}${clamped.toFixed(1)}%`, positive: clamped >= 0 };
};

interface StatCardDef {
  title: string;
  accent: CardAccent;
  loading?: boolean;
  value?: string;
  secondaryValue?: string;
  secondary?: string;
  badge?: React.ReactNode;
  trend?: { value: string; positive: boolean; direction?: "up" | "down" };
  context?: string;
  onClick?: () => void;
}

const useDarkMode = () => {
  const [isDark, setIsDark] = useState(() =>
    document.documentElement.classList.contains("dark")
  );

  useEffect(() => {
    const observer = new MutationObserver(() => {
      setIsDark(document.documentElement.classList.contains("dark"));
    });
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["class"],
    });
    return () => observer.disconnect();
  }, []);

  return isDark;
};

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const [flowMode, setFlowMode] = useState<"daily" | "weekly" | "monthly">("weekly");
  const [financeMode, setFinanceMode] = useState<"daily" | "weekly" | "monthly">("daily");
  const isDark = useDarkMode();

  const {
    data: stats,
    isLoading: statsLoading,
    isError: statsError,
    dataUpdatedAt,
  } = useDashboardStats();
  const { data: flowData } = usePatientsFlow(flowMode);
  const financeFilter = React.useMemo<ReportFilter>(
    () => ({ filter_type: financeMode }),
    [financeMode],
  );
  const { data: financeSummary, isLoading: financeLoading } =
    useReportSummary(financeFilter);
  const { data: recentPatients, refetch: refetchRecentPatients } =
    useRecentPatients(4);
  const updateStatusMutation = useUpdateVisitStatus();

  const handleUpdateStatus = useCallback(
    async (visitId: string, newStatus: string) => {
      await updateStatusMutation.mutateAsync({
        id: visitId,
        status: newStatus as "Open" | "Completed" | "Cancelled",
      });
      toast.success({ title: t("dashboard.notifications.statusUpdated", "Status updated to {{status}}", { status: newStatus }) });
    },
    [updateStatusMutation],
  );

  const statCards: StatCardDef[] = React.useMemo(() => {
    if (statsLoading) {
      return [
        { title: t("dashboard.stats.dailyRevenue", "Daily Revenue"), accent: "green", loading: true },
        { title: t("dashboard.stats.patientsToday", "Patients Today"), accent: "blue", loading: true },
        { title: t("dashboard.stats.outstandingBalance", "Outstanding Balance"), accent: "orange", loading: true },
        { title: t("dashboard.stats.proceduresPerformed", "Procedures Performed"), accent: "purple", loading: true },
        { title: t("dashboard.stats.dailyExpenses", "Daily Expenses"), accent: "red", loading: true },
        { title: t("dashboard.stats.netToday", "Net Today"), accent: "teal", loading: true },
      ];
    }

    if (statsError || !stats) {
      return [
        { title: t("dashboard.stats.dailyRevenue", "Daily Revenue"), value: "0 AFN", accent: "green" },
        { title: t("dashboard.stats.patientsToday", "Patients Today"), value: "0", accent: "blue" },
        { title: t("dashboard.stats.outstandingBalance", "Outstanding Balance"), value: "0 AFN", secondary: `0 ${t("dashboard.invoices", "invoices")}`, accent: "orange", onClick: () => navigate("/billing?filter=outstanding") },
        { title: t("dashboard.stats.proceduresPerformed", "Procedures Performed"), value: "00", accent: "purple" },
        { title: t("dashboard.stats.dailyExpenses", "Daily Expenses"), value: "0 AFN", accent: "red", onClick: () => navigate("/expenses") },
        { title: t("dashboard.stats.netToday", "Net Today"), value: "0 AFN", accent: "teal" },
      ];
    }

    const balance = stats.outstanding_balance;
    const outstandingBadge =
      balance === 0
        ? <Badge variant="success" className="text-[10px] sm:text-xs font-bold px-2 py-0.5">{t("dashboard.clear", "Clear")}</Badge>
        : balance < 10000
          ? <Badge variant="info" className="text-[10px] sm:text-xs font-bold px-2 py-0.5">{t("dashboard.low", "Low")}</Badge>
          : balance < 50000
            ? <Badge variant="warning" className="text-[10px] sm:text-xs font-bold px-2 py-0.5">{t("dashboard.medium", "Medium")}</Badge>
            : <Badge variant="destructive" className="text-[10px] sm:text-xs font-bold px-2 py-0.5">{t("dashboard.high", "High")}</Badge>;

    return [
      {
        title: t("dashboard.stats.dailyRevenue", "Daily Revenue"),
        value: formatAFN(stats.daily_revenue_afn),
        secondaryValue: formatUSD(stats.daily_revenue_usd),
        accent: "green",
        trend: computeTrend(stats.daily_revenue_afn, stats.yesterday_revenue_afn),
      },
      {
        title: t("dashboard.stats.patientsToday", "Patients Today"),
        value: String(stats.patients_today),
        accent: "blue",
        trend: computeTrend(stats.patients_today, stats.yesterday_patients),
        context: t("dashboard.todayAppointments", "today's appointments"),
      },
      {
        title: t("dashboard.stats.outstandingBalance", "Outstanding Balance"),
        value: formatAFN(stats.outstanding_balance_afn),
        secondaryValue: formatUSD(stats.outstanding_balance_usd),
        secondary: t("dashboard.invoiceCount", { count: stats.outstanding_invoices_count }),
        accent: "orange",
        badge: outstandingBadge,
        onClick: () => navigate("/billing?filter=outstanding"),
      },
      {
        title: t("dashboard.stats.proceduresPerformed", "Procedures Performed"),
        value: String(stats.procedures_performed).padStart(2, "0"),
        accent: "purple",
        trend: computeTrend(stats.procedures_performed, stats.yesterday_procedures),
        context: t("dashboard.stats.todayContext", "today"),
      },
      {
        title: t("dashboard.stats.dailyExpenses", "Daily Expenses"),
        value: formatAFN(stats.daily_expenses_afn),
        secondaryValue: formatUSD(stats.daily_expenses_usd),
        accent: "red",
        trend: (() => {
          const raw = computeTrend(stats.daily_expenses_afn, stats.yesterday_expenses_afn);
          if (!raw) return undefined;
          return {
            value: raw.value,
            positive: !raw.positive,
            direction: raw.positive ? "up" : "down",
          };
        })(),
        onClick: () => navigate("/expenses"),
      },
      {
        title: t("dashboard.stats.netToday", "Net Today"),
        value: formatAFN(stats.net_today_afn),
        secondaryValue: formatUSD(stats.net_today_usd),
        accent: "teal",
        trend: computeTrend(
          stats.net_today_afn,
          stats.yesterday_revenue_afn - stats.yesterday_expenses_afn,
        ),
        context: t("dashboard.stats.netContext", "revenue minus expenses"),
      },
    ];
  }, [stats, statsLoading, statsError, t]);

  const flowChartOptions: ApexOptions = React.useMemo(() => {
    const textColor = isDark ? "#6b7280" : "#9ca3af";
    const gridColor = isDark ? "rgba(75,85,99,0.15)" : "rgba(0,0,0,0.04)";
    return {
      chart: {
        type: "area",
        height: "100%",
        toolbar: { show: false },
        sparkline: { enabled: false },
        fontFamily: "Inter, system-ui, sans-serif",
        background: "transparent",
        animations: {
          enabled: true,
          easing: "easeinout",
          speed: 600,
          animateGradually: { enabled: true, delay: 150 },
        },
        dropShadow: {
          enabled: true,
          top: 4,
          left: 0,
          blur: 12,
          opacity: 0.06,
          color: "#000",
        },
      },
      stroke: {
        curve: "smooth",
        width: 2.5,
        lineCap: "round",
      },
      grid: {
        show: true,
        borderColor: gridColor,
        strokeDashArray: 4,
        position: "back",
        xaxis: { lines: { show: false } },
        yaxis: { lines: { show: true } },
        padding: { top: 10, right: 10, bottom: 0, left: 10 },
      },
      colors: ["#3b82f6"],
      fill: {
        type: "gradient",
        gradient: {
          shade: "light",
          type: "vertical",
          shadeIntensity: 0.35,
          opacityFrom: 0.28,
          opacityTo: 0.04,
          stops: [0, 80, 100],
        },
      },
      xaxis: {
        categories: (flowData ?? []).map((d) => d.label),
        axisBorder: { show: false },
        axisTicks: { show: false },
        labels: {
          style: {
            fontSize: "11px",
            fontWeight: 500,
            colors: textColor,
          },
          offsetY: 6,
        },
        crosshairs: {
          show: true,
          stroke: { color: gridColor, width: 1, dashArray: 4 },
        },
      },
      yaxis: {
        show: true,
        axisBorder: { show: false },
        axisTicks: { show: false },
        min: 0,
        labels: {
          style: {
            fontSize: "11px",
            fontWeight: 500,
            colors: textColor,
          },
          offsetX: -6,
        },
      },
      legend: {
        show: true,
        position: "top",
        horizontalAlign: "right",
        fontSize: "12px",
        fontWeight: 500,
        fontFamily: "Inter, system-ui, sans-serif",
        markers: {
          radius: 2,
          strokeWidth: 0,
          size: 8,
          offsetX: -2,
        },
        itemMargin: { horizontal: 12, vertical: 0 },
        labels: {
          colors: isDark ? "#d1d5db" : "#374151",
          useSeriesColors: false,
        },
      },
      tooltip: {
        shared: true,
        intersect: false,
        theme: isDark ? "dark" : "light",
        style: { fontSize: "12px" },
        marker: { show: false },
        x: { show: false },
        custom: function ({ series, dataPointIndex, w }: any) {
          const label = w.globals.labels[dataPointIndex];
          const visits = series[0][dataPointIndex];
          return `
            <div style="
              padding: 12px 16px;
              border-radius: 12px;
              background: ${isDark ? "#1f2937" : "#ffffff"};
              box-shadow: 0 8px 32px rgba(0,0,0,${isDark ? "0.4" : "0.1"}), 0 1px 3px rgba(0,0,0,${isDark ? "0.2" : "0.06"});
              border: 1px solid ${isDark ? "#374151" : "#f3f4f6"};
              font-family: Inter, system-ui, sans-serif;
            ">
              <div style="
                font-size: 11px;
                font-weight: 600;
                color: ${isDark ? "#9ca3af" : "#6b7280"};
                margin-bottom: 8px;
                letter-spacing: 0.03em;
              ">${label}</div>
              <div style="display: flex; flex-direction: column; gap: 6px;">
                <div style="display: flex; align-items: center; gap: 8px;">
                  <span style="width: 8px; height: 8px; border-radius: 50%; background: #3b82f6; flex-shrink: 0;"></span>
                  <span style="font-size: 12px; color: ${isDark ? "#d1d5db" : "#374151"}; font-weight: 500;">${t("dashboard.visits", "Visits")}</span>
                  <span style="font-size: 13px; font-weight: 700; color: ${isDark ? "#f9fafb" : "#111827"}; margin-left: auto;">${visits}</span>
                </div>
              </div>
            </div>
          `;
        },
      },
      markers: {
        size: 0,
        hover: {
          size: 5,
          sizeOffset: 3,
        },
      },
      dataLabels: { enabled: false },
      responsive: [
        {
          breakpoint: 640,
          options: {
            chart: { height: 220 },
            legend: { fontSize: "10px" },
          },
        },
      ],
    };
  }, [flowData, isDark, t]);

  const flowChartSeries: ApexOptions["series"] = React.useMemo(
    () => [
      {
        name: t("dashboard.visits", "Visits"),
        data: (flowData ?? []).map((d) => d.visits),
      },
    ],
    [flowData, t],
  );

  const financeSlices = React.useMemo(() => {
    if (!financeSummary) return [];
    return [
      {
        label: t("reports.stats.revenue", "Revenue"),
        value: financeSummary.revenue_this_month_afn,
        color: FINANCE_COLORS.revenue,
      },
      {
        label: t("reports.stats.expenses", "Expenses"),
        value: financeSummary.expenses_this_month_afn,
        color: FINANCE_COLORS.expenses,
      },
      {
        label: t("reports.stats.outstanding", "Outstanding"),
        value: financeSummary.outstanding_balance_afn,
        color: FINANCE_COLORS.outstanding,
      },
    ].filter((slice) => slice.value > 0);
  }, [financeSummary, t]);

  const netProfit =
    (financeSummary?.revenue_this_month_afn ?? 0) -
    (financeSummary?.expenses_this_month_afn ?? 0);

  return (
    <div className="space-y-4 sm:space-y-5 xl:space-y-6">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-3 sm:gap-4">
        <div className="space-y-0.5">
          <p className="text-[10px] sm:text-xs font-bold uppercase tracking-[0.15em] text-teal-600 dark:text-teal-400">
            {t("dashboard.welcome", "WELCOME BACK")}
          </p>
          <h1 className="text-xl sm:text-2xl lg:text-3xl font-extrabold text-gray-900 dark:text-white tracking-tight">
            {t("dashboard.title", "Clinic Dashboard")}
          </h1>
          <div className="flex items-center gap-2 mt-1">
            <span className="text-[10px] text-gray-400 dark:text-gray-500">
              {t("dashboard.lastUpdated", "Last updated")}:{" "}
              {dataUpdatedAt > 0 ? format(dataUpdatedAt, "HH:mm:ss") : "--:--:--"}
            </span>
            <div
              className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"
              title={t("dashboard.realTimeSync", "Real-time sync")}
            />
          </div>
        </div>
        <div className="flex items-center gap-2 sm:gap-3 w-full md:w-auto">
          <Button
            onClick={() => navigate("/patients/new")}
            className="gap-2 px-4 sm:px-6 py-2 sm:py-2.5 text-xs sm:text-sm font-bold text-white shadow-lg shadow-primary/20 hover:shadow-primary/30 transition-all duration-300 hover:scale-[1.02] active:scale-[0.98] w-full xl:w-auto"
          >
            <PlusIcon size="md" />
            <span>{t("nav.newPatient", "Add New Patient")}</span>
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
        {statCards.map((stat) => (
          <StatCard
            key={stat.title}
            title={stat.title}
            value={stat.value ?? ""}
            accent={stat.accent}
            badge={stat.badge}
            trend={stat.trend}
            loading={stat.loading ?? false}
            secondary={stat.secondary}
            secondaryValue={stat.secondaryValue}
            context={stat.context}
            onClick={stat.onClick}
          />
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 sm:gap-6">
        <ChartCard
          title={t("dashboard.patientsFlow", "Patients Flow")}
          className="col-span-1 lg:col-span-7"
          action={
            <Select
              value={flowMode}
              onChange={(e) => setFlowMode(e.target.value as "daily" | "weekly" | "monthly")}
              className="w-auto text-xs sm:text-sm py-1 px-2 min-w-30 cursor-pointer"
            >
              <option value="daily">{t("reports.filter.today", "Today")}</option>
              <option value="weekly">{t("reports.filter.thisWeek", "This Week")}</option>
              <option value="monthly">{t("reports.filter.thisMonth", "This Month")}</option>
            </Select>
          }
        >
          <div className="h-56 sm:h-64 lg:h-72 w-full">
            <Chart
              options={flowChartOptions}
              series={flowChartSeries}
              type="area"
              height="100%"
            />
          </div>
        </ChartCard>

        <ChartCard
          title={t("dashboard.financeDistribution", "Finance Overview")}
          className="col-span-1 lg:col-span-5"
          action={
            <Select
              value={financeMode}
              onChange={(e) => setFinanceMode(e.target.value as "daily" | "weekly" | "monthly")}
              className="w-auto text-xs sm:text-sm py-1 px-2 min-w-30 cursor-pointer"
            >
              <option value="daily">{t("reports.filter.today", "Today")}</option>
              <option value="weekly">{t("reports.filter.thisWeek", "This Week")}</option>
              <option value="monthly">{t("reports.filter.thisMonth", "This Month")}</option>
            </Select>
          }
        >
          <div>
            <div className="h-44 sm:h-52 lg:h-60 w-full">
              {financeLoading ? (
                <div className="h-full w-full animate-pulse rounded-xl bg-gray-100 dark:bg-gray-700/40" />
              ) : financeSlices.length > 0 ? (
                <Chart
                  options={{
                    chart: {
                      type: "pie",
                      height: "100%",
                      toolbar: { show: false },
                      fontFamily: "Inter, system-ui, sans-serif",
                      background: "transparent",
                      animations: {
                        enabled: true,
                        easing: "easeinout",
                        speed: 600,
                      },
                    },
                    colors: financeSlices.map((s) => s.color),
                    labels: financeSlices.map((s) => s.label),
                    legend: {
                      show: true,
                      position: "bottom",
                      fontSize: "12px",
                      fontWeight: 500,
                      fontFamily: "Inter, system-ui, sans-serif",
                      markers: {
                        strokeWidth: 0,
                        size: 8,
                        offsetX: -2,
                      },
                      itemMargin: { horizontal: 8, vertical: 4 },
                      labels: {
                        colors: isDark ? "#d1d5db" : "#374151",
                        useSeriesColors: false,
                      },
                    },
                    dataLabels: {
                      enabled: true,
                      formatter: (val: number) => `${val.toFixed(1)}%`,
                      style: {
                        fontSize: "11px",
                        fontWeight: 600,
                        colors: [isDark ? "#e5e7eb" : "#374151"],
                      },
                      dropShadow: { enabled: false },
                    },
                    tooltip: {
                      theme: isDark ? "dark" : "light",
                      style: { fontSize: "12px" },
                      y: { formatter: (val: number) => formatAFN(val) },
                    },
                    stroke: {
                      width: 2,
                      colors: [isDark ? "#1f2937" : "#ffffff"],
                    },
                    responsive: [
                      {
                        breakpoint: 640,
                        options: {
                          legend: { fontSize: "10px" },
                        },
                      },
                    ],
                  }}
                  series={financeSlices.map((s) => s.value)}
                  type="pie"
                  height="100%"
                />
              ) : (
                <div className="flex flex-col items-center justify-center h-full text-gray-400 dark:text-gray-500 gap-2">
                  <span className="text-xs font-medium">{t("dashboard.noDataAvailable", "No data available")}</span>
                </div>
              )}
            </div>
            {financeSummary && financeSlices.length > 0 && (
              <div className="mt-3 border-t border-gray-100 dark:border-gray-700 pt-2.5 text-center">
                <div className="text-[11px] text-gray-500 dark:text-gray-400">
                  {t("dashboard.netProfitFormula", "Net Profit = Revenue - Expenses")}
                </div>
                <div dir="ltr" className="mt-1 text-xs font-semibold text-gray-900 dark:text-white">
                  {formatAFN(financeSummary.revenue_this_month_afn)} - {formatAFN(financeSummary.expenses_this_month_afn)} ={" "}
                  <span className={netProfit >= 0 ? "text-emerald-600 dark:text-emerald-400" : "text-red-600 dark:text-red-400"}>
                    {formatAFN(netProfit)}
                  </span>
                </div>
              </div>
            )}
          </div>
        </ChartCard>
      </div>

      <RecentPatientsTable
        patients={recentPatients ?? []}
        onUpdateStatus={handleUpdateStatus}
        onRefetch={refetchRecentPatients}
      />

      <div className="flex items-center justify-between text-[10px] text-gray-400 dark:text-gray-500 px-1">
        <div className="flex items-center gap-3">
          <span>{t("dashboard.liveDataFromDb", "Live data from database")}</span>
          <div className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse" />
        </div>
        <div className="flex items-center gap-1.5">
          <ActivityIcon size="xs" />
          <span>{t("dashboard.autoRefresh", "Auto-refresh every 5 min")}</span>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
