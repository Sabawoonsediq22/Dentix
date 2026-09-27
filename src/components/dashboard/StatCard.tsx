import React from "react";
import { cn } from "../../lib/utils";
import { TrendingDownIcon, TrendingUpIcon } from "../../shared/icons/icons";
import SparklineChart from "../charts/SparklineChart";

export type CardAccent = "green" | "blue" | "orange" | "purple" | "red" | "teal";

interface StatCardProps {
  title: string;
  value: string;
  accent: CardAccent;
  badge?: React.ReactNode;
  trend?: { value: string; positive: boolean };
  loading?: boolean;
  secondary?: string;
  secondaryValue?: string;
  context?: string;
  sparklineData?: { day: string; value: number }[];
  className?: string;
  onClick?: () => void;
}

const accentConfig: Record<CardAccent, { border: string; bg: string; glow: string; sparkline: string }> = {
  green: {
    border: "border-t-emerald-500",
    bg: "from-emerald-500/[0.03] to-transparent dark:from-emerald-500/[0.06]",
    glow: "group-hover:shadow-emerald-500/8",
    sparkline: "#10b981",
  },
  blue: {
    border: "border-t-blue-500",
    bg: "from-blue-500/[0.03] to-transparent dark:from-blue-500/[0.06]",
    glow: "group-hover:shadow-blue-500/8",
    sparkline: "#3b82f6",
  },
  orange: {
    border: "border-t-orange-500",
    bg: "from-orange-500/[0.03] to-transparent dark:from-orange-500/[0.06]",
    glow: "group-hover:shadow-orange-500/8",
    sparkline: "#f97316",
  },
  purple: {
    border: "border-t-purple-500",
    bg: "from-purple-500/[0.03] to-transparent dark:from-purple-500/[0.06]",
    glow: "group-hover:shadow-purple-500/8",
    sparkline: "#a855f7",
  },
  red: {
    border: "border-t-red-500",
    bg: "from-red-500/[0.03] to-transparent dark:from-red-500/[0.06]",
    glow: "group-hover:shadow-red-500/8",
    sparkline: "#ef4444",
  },
  teal: {
    border: "border-t-teal-600",
    bg: "from-teal-600/[0.03] to-transparent dark:from-teal-600/[0.06]",
    glow: "group-hover:shadow-teal-600/8",
    sparkline: "#0d9488",
  },
};

const StatCard: React.FC<StatCardProps> = ({
  title,
  value,
  accent,
  badge,
  trend,
  loading,
  secondary,
  secondaryValue,
  context,
  sparklineData,
  className,
  onClick,
}) => {
  const config = accentConfig[accent];

  return (
    <div
      onClick={onClick}
      className={cn(
        "group relative rounded-2xl border border-gray-200/80 dark:border-gray-700/50",
        "bg-white dark:bg-gray-800/60 backdrop-blur-sm",
        "shadow-sm dark:shadow-gray-900/20",
        "transition-all duration-300 ease-out",
        "hover:shadow-xl hover:shadow-gray-200/50 dark:hover:shadow-gray-900/40",
        "hover:-translate-y-0.5",
        "border-t-[3px]",
        config.border,
        config.glow,
        onClick && "cursor-pointer",
        className,
      )}
    >
      <div className={cn("absolute inset-0 rounded-2xl bg-linear-to-b opacity-0 group-hover:opacity-100 transition-opacity duration-300", config.bg)} />

      <div className="relative p-5 sm:p-6">
        <div className="flex items-start justify-between gap-3">
          <div className="flex-1 min-w-0">
            <p className="text-xs font-semibold uppercase tracking-wider text-gray-400 dark:text-gray-500">
              {title}
            </p>

            <div className="mt-2">
              {loading ? (
                <div className="flex items-center gap-2">
                  <div className="h-8 w-24 rounded-lg bg-gray-100 dark:bg-gray-700 animate-pulse" />
                </div>
              ) : (
                <div className="flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-1">
                  <p className="text-lg sm:text-xl font-bold tracking-tight text-gray-900 dark:text-white leading-none tabular-nums">
                    {value}
                  </p>
                  {secondaryValue && (
                    <span className="text-sm sm:text-lg text-gray-700 dark:text-gray-400 tabular-nums leading-none mt-1 sm:mt-0 font-bold">
                      {secondaryValue}
                    </span>
                  )}
                </div>
              )}
            </div>

            {secondary && !loading && (
              <p className="text-sm text-gray-500 dark:text-gray-400 font-medium mt-1.5">
                {secondary}
              </p>
            )}

            <div className="flex items-center gap-2 mt-2">
              {trend && !loading && (
                <span
                  className={cn(
                    "inline-flex items-center gap-1 text-xs font-bold px-2 py-0.5 rounded-full",
                    trend.positive
                      ? "text-emerald-700 bg-emerald-50 dark:text-emerald-400 dark:bg-emerald-500/10"
                      : "text-red-600 bg-red-50 dark:text-red-400 dark:bg-red-500/10"
                  )}
                >
                  {trend.positive ? (
                    <TrendingUpIcon size="xs" />
                  ) : (
                    <TrendingDownIcon size="xs" />
                  )}
                  {trend.value}
                </span>
              )}
              {context && !loading && (
                <span className="text-[11px] text-gray-400 dark:text-gray-500 font-medium">
                  {context}
                </span>
              )}
            </div>
          </div>

        </div>

        {sparklineData && sparklineData.length > 0 && !loading && (
          <div className="mt-3 -mx-1">
            <SparklineChart
              data={sparklineData}
              color={config.sparkline}
              height={44}
            />
          </div>
        )}

        {badge && !loading && (
          <div className="mt-3 pt-3 border-t border-gray-100 dark:border-gray-700/50">
            {badge}
          </div>
        )}
      </div>
    </div>
  );
};

export default StatCard;
