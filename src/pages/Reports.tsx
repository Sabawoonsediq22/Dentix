import React, { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Card, CardContent, CardHeader, CardTitle, LoadingSpinner, DatePicker, Button } from "../components/ui";
import { useReportSummary, useMonthlyRevenue } from "../hooks/useReports";
import Chart from "react-apexcharts";
import { PatientIcon, ToothIcon, DownloadIcon, FileIcon, CurrencyIcon } from "../shared/icons/icons";
import type { MonthlyRevenuePoint, DailyTrendPoint, ReportFilter } from "../types/ApiTypes";
import { exportPatientsReport, exportFinancialReport, exportTreatmentReport, exportExpensesReport } from "../lib/export";
import type { ReportFormat } from "../lib/export";
import { toast } from "../lib/toast-utils";
import SparklineChart from "../components/charts/SparklineChart";

const MONTH_NAMES = [
  "Jan", "Feb", "Mar", "Apr", "May", "Jun",
  "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

const useDarkMode = () => {
  const [isDark, setIsDark] = useState(() =>
    document.documentElement.classList.contains("dark")
  );

  React.useEffect(() => {
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

interface StatCardProps {
  title: string;
  value: string;
  valueUsd?: string;
  trendData?: DailyTrendPoint[];
  trendColor?: string;
  change?: {
    value: string;
    positive?: boolean;
  };
}

const StatCard: React.FC<StatCardProps> = ({ title, value, valueUsd, trendData, trendColor = "#0d9488", change }) => (
  <div className="rounded-xl border border-gray-200 bg-white p-4 sm:p-5 shadow-sm hover:shadow-md transition-shadow dark:border-gray-700 dark:bg-gray-800 flex flex-col">
    <div className="flex items-start justify-between">
      <div className="min-w-0 flex-1">
        <p className="text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
          {title}
        </p>
        <p className="mt-1.5 text-xl sm:text-2xl font-bold text-gray-900 dark:text-white">
          {value}
        </p>
        {valueUsd && (
          <span className="inline-flex items-center text-[11px] sm:text-xl font-medium text-gray-700 dark:text-gray-400 bg-gray-100/80 dark:bg-gray-700/50 border border-gray-200/60 dark:border-gray-600/40 rounded-full px-2.5 py-0.5 mt-1.5 tabular-nums">
            {valueUsd}
          </span>
        )}
        {change && (
          <div className="mt-2">
            <span className={`text-xs font-semibold inline-flex items-center gap-1 px-2 py-0.5 rounded-full ${
              change.positive === undefined
                ? "text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700/50"
                : change.positive
                  ? "text-emerald-700 bg-emerald-50 dark:text-emerald-400 dark:bg-emerald-500/10"
                  : "text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-500/10"
            }`}>
              {change.positive === true && "↑"}
              {change.positive === false && "↓"}
              {change.value}
            </span>
          </div>
        )}
      </div>
    </div>
    {trendData && (
      <div className="mt-3 -mx-1">
        <SparklineChart data={trendData} color={trendColor} height={50} />
      </div>
    )}
  </div>
);

const formatAFN = (val: number) =>
  val.toLocaleString("en-US", { minimumFractionDigits: 0, maximumFractionDigits: 0 }) + " AFN";

const formatUSD = (val: number) =>
  "$" + val.toLocaleString("en-US", { minimumFractionDigits: 2, maximumFractionDigits: 2 });

const formatMonth = (monthStr: string) => {
  const parts = monthStr.split("-");
  if (parts.length === 3) {
    const month = MONTH_NAMES[parseInt(parts[1], 10) - 1];
    return month ? `${month} ${parseInt(parts[2], 10)}` : monthStr;
  }
  return MONTH_NAMES[parseInt(parts[1], 10) - 1] || monthStr;
};

const toDateString = (date: Date) =>
  `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, "0")}-${String(date.getDate()).padStart(2, "0")}`;

const Reports: React.FC = () => {
  const { t } = useTranslation();
  const isDark = useDarkMode();
  const [exporting, setExporting] = useState<string | null>(null);
  const [format, setFormat] = useState<ReportFormat>("pdf");
  const [filterType, setFilterType] = useState<string>("monthly");
  const [customStartDate, setCustomStartDate] = useState<Date | undefined>(undefined);
  const [customEndDate, setCustomEndDate] = useState<Date | undefined>(undefined);

  const reportFilter = useMemo<ReportFilter>(() => {
    if (filterType === "custom") {
      const start = customStartDate ?? customEndDate;
      const end = customEndDate;
      return {
        filter_type: "custom",
        start_date: start ? toDateString(start) : undefined,
        end_date: end ? toDateString(end) : undefined,
      };
    }
    return { filter_type: filterType };
  }, [filterType, customStartDate, customEndDate]);

  const { data: summary, isLoading, error } = useReportSummary(reportFilter);
  const { data: monthlyRevenue, isLoading: revenueLoading } = useMonthlyRevenue(reportFilter);

  const statCards = useMemo(() => {
    if (!summary) return [];

    const sumTrend = (data: DailyTrendPoint[]) =>
      data.reduce((acc, d) => acc + d.value, 0);

    const currentVisits = sumTrend(summary.visits_trend);
    const currentRevenue = sumTrend(summary.revenue_trend);
    const currentExpenses = sumTrend(summary.expenses_trend);
    const currentOutstanding = sumTrend(summary.outstanding_trend);
    const currentNet = currentRevenue - currentExpenses;
    const prevNet = summary.prev_revenue - summary.prev_expenses;

    const netTrend: DailyTrendPoint[] = summary.revenue_trend.map((point, idx) => ({
      day: point.day,
      value: point.value - (summary.expenses_trend[idx]?.value ?? 0),
    }));

    const pct = (cur: number, prev: number): { value: string; positive: boolean } | undefined => {
      if (prev <= 0) return undefined;
      const change = ((cur - prev) / prev) * 100;
      const periodLabel = filterType === "daily"
        ? t("reports.filter.vsYesterday", "vs yesterday")
        : filterType === "weekly"
          ? t("reports.filter.vsLastWeek", "vs last week")
          : filterType === "custom"
            ? t("reports.filter.vsPreviousPeriod", "vs previous period")
            : t("reports.filter.vsLastMonth", "vs last month");
      return {
        value: `${Math.abs(change).toFixed(1)}% ${periodLabel}`,
        positive: change >= 0,
      };
    };

    return [
      {
        title: t("reports.stats.activePatients", "Active Patients"),
        value: String(summary.active_patients),
        trendData: summary.active_patients_trend,
        trendColor: "#3b82f6",
        change: pct(summary.active_patients, summary.prev_active_patients),
      },
      {
        title: t("reports.stats.totalVisits", "Total Visits"),
        value: String(summary.total_visits_this_month),
        trendData: summary.visits_trend,
        trendColor: "#0d9488",
        change: pct(currentVisits, summary.prev_total_visits),
      },
      {
        title: t("reports.stats.revenue", "Revenue"),
        value: formatAFN(summary.revenue_this_month_afn),
        valueUsd: formatUSD(summary.revenue_this_month_usd),
        trendData: summary.revenue_trend,
        trendColor: "#22c55e",
        change: pct(currentRevenue, summary.prev_revenue),
      },
      {
        title: t("reports.stats.expenses", "Expenses"),
        value: formatAFN(summary.expenses_this_month_afn),
        valueUsd: formatUSD(summary.expenses_this_month_usd),
        trendData: summary.expenses_trend,
        trendColor: "#ef4444",
        change: pct(currentExpenses, summary.prev_expenses),
      },
      {
        title: t("reports.stats.netProfit", "Net Profit"),
        value: formatAFN(summary.revenue_this_month_afn - summary.expenses_this_month_afn),
        valueUsd: formatUSD(summary.revenue_this_month_usd - summary.expenses_this_month_usd),
        trendData: netTrend,
        trendColor: "#8b5cf6",
        change: pct(currentNet, prevNet),
      },
      {
        title: t("reports.stats.outstanding", "Outstanding"),
        value: formatAFN(summary.outstanding_balance_afn),
        valueUsd: formatUSD(summary.outstanding_balance_usd),
        trendData: summary.outstanding_trend,
        trendColor: "#f59e0b",
        change: pct(currentOutstanding, summary.prev_outstanding),
      },
    ];
  }, [summary, t, filterType]);

  if (isLoading || revenueLoading) {
    return (
      <div className="flex h-full items-center justify-center">
        <LoadingSpinner size="lg" text={t("reports.loading", "Loading reports...")} />
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-lg text-red-500">
          {t("reports.error", "Error loading reports")}: {String(error)}
        </div>
      </div>
    );
  }

  const chartData: (MonthlyRevenuePoint & { monthLabel: string; revenueAfn: number; revenueUsd: number })[] =
    monthlyRevenue && monthlyRevenue.length > 0
      ? monthlyRevenue.map((p) => ({
          ...p,
          monthLabel: formatMonth(p.month),
          revenueAfn: p.revenue_afn,
          revenueUsd: p.revenue_usd,
        }))
      : summary
        ? [
            {
              month: new Date().toISOString().slice(0, 7),
              revenue: summary.revenue_this_month,
              revenue_afn: summary.revenue_this_month_afn,
              revenue_usd: summary.revenue_this_month_usd,
              expenses: summary.expenses_this_month,
              expenses_afn: summary.expenses_this_month_afn,
              expenses_usd: summary.expenses_this_month_usd,
              revenueAfn: summary.revenue_this_month_afn,
              revenueUsd: summary.revenue_this_month_usd,
              monthLabel: formatMonth(new Date().toISOString().slice(0, 7)),
            },
          ]
        : [];

  const handleExport = async (type: string, fn: (format: ReportFormat) => Promise<void>) => {
    setExporting(type);
    try {
      await fn(format);
      toast.success({ title: t("reports.export.success", "Report exported successfully") });
    } catch (err) {
      toast.error({ title: t("reports.export.error", "Failed to export report"), description: String(err) });
    } finally {
      setExporting(null);
    }
  };

  const visitStatusData = summary
    ? [
        {
          name: t("reports.status.completed", "Completed"),
          value: summary.completed_visits_this_month,
          fill: "#22c55e",
        },
        {
          name: t("reports.status.cancelled", "Cancelled"),
          value: summary.cancelled_visits_this_month,
          fill: "#ef4444",
        },
        {
          name: t("reports.status.active", "Active"),
          value: Math.max(0, (summary.total_visits_this_month || 0) - summary.completed_visits_this_month - summary.cancelled_visits_this_month),
          fill: "#3b82f6",
        },
      ]
    : [];

  return (
    <div className="space-y-4 sm:space-y-6">
      <div>
        <p className="text-xs font-semibold uppercase tracking-wider text-teal-600 dark:text-teal-400">
          {t("reports.subtitle", "CLINIC ANALYTICS")}
        </p>
        <h1 className="mt-1 text-2xl sm:text-3xl font-bold text-gray-900 dark:text-white">
          {t("reports.title", "Reports")}
        </h1>
      </div>

      <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3 my-6">
        <div className="flex items-center gap-1 bg-gray-200 dark:bg-gray-800 rounded-lg p-0.5 px-1 py-1 ms-auto">
          {[
            { value: "daily", label: t("reports.filter.today", "Today") },
            { value: "weekly", label: t("reports.filter.thisWeek", "This Week") },
            { value: "monthly", label: t("reports.filter.thisMonth", "This Month") },
            { value: "custom", label: t("reports.filter.custom", "Custom Range") },
          ].map((tab) => (
            <Button
              key={tab.value}
              variant={filterType === tab.value ? "default" : "ghost"}
              size="sm"
              onClick={() => setFilterType(tab.value)}
              className="cursor-pointer"
            >
              {tab.label}
            </Button>
          ))}
        </div>
        {filterType === "custom" && (
          <div className="flex items-end gap-3 w-full sm:w-auto">
            <div className="flex-1 min-w-0">
              <label className="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5">
                {t("reports.filter.startDate", "Start Date")}
              </label>
              <DatePicker
                value={customStartDate}
                onChange={setCustomStartDate}
                placeholder={t("reports.filter.selectStart", "Select start")}
                maxDate={customEndDate}
                className="w-full"
              />
            </div>
            <div className="flex-1 min-w-0">
              <label className="block text-xs font-medium text-gray-500 dark:text-gray-400 mb-1.5">
                {t("reports.filter.endDate", "End Date")}
              </label>
              <DatePicker
                value={customEndDate}
                onChange={setCustomEndDate}
                placeholder={t("reports.filter.selectEnd", "Select end")}
                minDate={customStartDate}
                className="w-full"
              />
            </div>
          </div>
        )}
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4 md:gap-6">
        {statCards.map((card, idx) => (
          <StatCard key={idx} {...card} />
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 sm:gap-6">
        <Card>
          <CardHeader>
            <CardTitle className="text-base sm:text-lg font-semibold">
              {t("reports.charts.revenueExpenseTrend", "Revenue vs Expenses")}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-56 sm:h-64 w-full">
              <Chart
                options={{
                  chart: {
                    type: "bar",
                    height: "100%",
                    toolbar: { show: false },
                    fontFamily: "Inter, system-ui, sans-serif",
                    background: "transparent",
                    animations: {
                      enabled: true,
                      easing: "easeinout",
                      speed: 700,
                    },
                  },
                  plotOptions: {
                    bar: {
                      borderRadius: 6,
                      columnWidth: "60%",
                      borderRadiusApplication: "end",
                      borderRadiusWhenStacked: "last",
                    },
                  },
                  stroke: {
                    show: true,
                    width: 0,
                    colors: ["transparent"],
                  },
                  grid: {
                    show: true,
                    borderColor: isDark ? "rgba(75,85,99,0.15)" : "rgba(0,0,0,0.04)",
                    strokeDashArray: 4,
                    position: "back",
                    xaxis: { lines: { show: false } },
                    yaxis: { lines: { show: true } },
                    padding: { top: 10, right: 10, bottom: 0, left: 10 },
                  },
                  colors: ["#0d9488", "#60a5fa", "#ef4444", "#f59e0b"],
                  fill: {
                    type: "gradient",
                    gradient: {
                      shade: "light",
                      type: "vertical",
                      shadeIntensity: 0.3,
                      opacityFrom: 1,
                      opacityTo: 0.85,
                      stops: [0, 100],
                    },
                  },
                  xaxis: {
                    categories: chartData.map((d) => d.monthLabel),
                    axisBorder: { show: false },
                    axisTicks: { show: false },
                    labels: {
                      style: {
                        fontSize: "11px",
                        fontWeight: 500,
                        colors: isDark ? "#6b7280" : "#9ca3af",
                      },
                      offsetY: 6,
                    },
                  },
                  yaxis: {
                    show: true,
                    axisBorder: { show: false },
                    axisTicks: { show: false },
                    labels: {
                      style: {
                        fontSize: "11px",
                        fontWeight: 500,
                        colors: isDark ? "#6b7280" : "#9ca3af",
                      },
                      offsetX: -6,
                      formatter: (val: number) => val >= 1000 ? `${(val / 1000).toFixed(0)}k` : String(val),
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
                    y: {
                      formatter: (val: number, opts: any) => {
                        return opts.seriesIndex % 2 === 0 ? formatAFN(val) : formatUSD(val);
                      },
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
                }}
                series={[
                  { name: t("reports.charts.revenueAfn", "Revenue AFN"), data: chartData.map((d) => d.revenueAfn) },
                  { name: t("reports.charts.revenueUsd", "Revenue USD"), data: chartData.map((d) => d.revenueUsd) },
                  { name: t("reports.charts.expensesAfn", "Expenses AFN"), data: chartData.map((d) => d.expenses_afn) },
                  { name: t("reports.charts.expensesUsd", "Expenses USD"), data: chartData.map((d) => d.expenses_usd) },
                ]}
                type="bar"
                height="100%"
              />
            </div>
            <div className="mt-2 flex items-center justify-center gap-4 text-xs text-gray-500 dark:text-gray-400">
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-full bg-[#0d9488]" />
                {t("reports.charts.revenueAfn", "Revenue AFN")}
              </span>
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-full bg-[#60a5fa]" />
                {t("reports.charts.revenueUsd", "Revenue USD")}
              </span>
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-full bg-[#ef4444]" />
                {t("reports.charts.expensesAfn", "Expenses AFN")}
              </span>
              <span className="flex items-center gap-1.5">
                <span className="inline-block h-2.5 w-2.5 rounded-full bg-[#f59e0b]" />
                {t("reports.charts.expensesUsd", "Expenses USD")}
              </span>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="text-base sm:text-lg font-semibold">
              {t("reports.charts.visitDistribution", "Visit Distribution")}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-56 sm:h-64 w-full">
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
                  colors: visitStatusData.map((d) => d.fill),
                  labels: visitStatusData.map((d) => d.name),
                  dataLabels: {
                    enabled: true,
                    formatter: (val: number) => `${val.toFixed(1)}%`,
                    style: {
                      fontSize: "11px",
                      fontWeight: 600,
                      colors: ["#fff"],
                    },
                    dropShadow: { enabled: false },
                  },
                  tooltip: {
                    theme: isDark ? "dark" : "light",
                    style: { fontSize: "12px" },
                    y: { formatter: (val: number) => String(val) },
                  },
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
                    itemMargin: { horizontal: 12, vertical: 0 },
                    labels: {
                      colors: isDark ? "#d1d5db" : "#374151",
                      useSeriesColors: false,
                    },
                  },
                  stroke: {
                    width: 2,
                    colors: [isDark ? "#1f2937" : "#ffffff"],
                  },
                  responsive: [
                    {
                      breakpoint: 640,
                      options: {
                        chart: { height: 220 },
                        legend: { fontSize: "10px" },
                      },
                    },
                  ],
                }}
                series={visitStatusData.map((d) => d.value)}
                type="pie"
                height="100%"
              />
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
            <CardTitle className="text-base sm:text-lg font-semibold">
              {t("reports.export.title", "Export Reports")}
            </CardTitle>
            <div className="flex items-center gap-2 self-start">
              <span className="text-xs font-medium text-gray-500 dark:text-gray-400">
                {t("reports.export.format", "Format:")}
              </span>
              <div className="flex rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
                <button
                  onClick={() => setFormat("pdf")}
                  className={`flex items-center gap-1.5 px-3 py-1.5 text-xs font-bold transition-all duration-200 cursor-pointer ${
                    format === "pdf"
                      ? "bg-primary text-white shadow-sm"
                      : "bg-white dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                  }`}
                >
                  <FileIcon size="sm" />
                  PDF
                </button>
                <button
                  onClick={() => setFormat("csv")}
                  className={`flex items-center gap-1.5 px-3 py-1.5 text-xs font-bold transition-all duration-200 cursor-pointer ${
                    format === "csv"
                      ? "bg-primary text-white shadow-sm"
                      : "bg-white dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                  }`}
                >
                  <FileIcon size="sm" />
                  CSV
                </button>
              </div>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
            <button
              className="flex flex-col items-center justify-center p-6 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/30 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed "
              onClick={() => handleExport("patients", exportPatientsReport)}
              disabled={exporting !== null}
            >
              {exporting === "patients" ? (
                <LoadingSpinner size="md" />
              ) : (
                <PatientIcon size="md" className="mb-3 text-primary" />
              )}
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {t("reports.export.patients", "Patient Report")}
              </span>
              <span className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {t("reports.export.patientsDesc", "Export patient records and statistics")}
              </span>
            </button>
            <button
              className="flex flex-col items-center justify-center p-6 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/30 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleExport("financial", exportFinancialReport)}
              disabled={exporting !== null}
            >
              {exporting === "financial" ? (
                <LoadingSpinner size="md" />
              ) : (
                <DownloadIcon size="md" className="mb-3 text-primary" />
              )}
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {t("reports.export.financial", "Financial Report")}
              </span>
              <span className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {t("reports.export.financialDesc", "Export invoices and payment history")}
              </span>
            </button>
            <button
              className="flex flex-col items-center justify-center p-6 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/30 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleExport("treatment", exportTreatmentReport)}
              disabled={exporting !== null}
            >
              {exporting === "treatment" ? (
                <LoadingSpinner size="md" />
              ) : (
                <ToothIcon size="md" className="mb-3 text-primary" />
              )}
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {t("reports.export.treatment", "Treatment Report")}
              </span>
              <span className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {t("reports.export.treatmentDesc", "Export treatment history and procedures")}
              </span>
            </button>
            <button
              className="flex flex-col items-center justify-center p-6 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700/30 transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleExport("expenses", exportExpensesReport)}
              disabled={exporting !== null}
            >
              {exporting === "expenses" ? (
                <LoadingSpinner size="md" />
              ) : (
                <CurrencyIcon size="md" className="mb-3 text-primary" />
              )}
              <span className="text-sm font-medium text-gray-900 dark:text-white">
                {t("reports.export.expenses", "Expense Report")}
              </span>
              <span className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                {t("reports.export.expensesDesc", "Export clinic expense records")}
              </span>
            </button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default Reports;
