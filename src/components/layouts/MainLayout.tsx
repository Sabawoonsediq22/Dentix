import React, { useState, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { NavLink, useLocation, useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { toast } from "sonner";
import {
  BillingIcon,
  CollapseIcon,
  CurrencyIcon,
  ExpandIcon,
  LayoutIcon,
  PatientIcon,
  ReportsIcon,
  ClipboardListIcon,
  MoonIcon,
  SettingsIcon,
  SunIcon,
} from "../../shared/icons/icons";
import { Button, Breadcrumbs } from "../ui/index";
import { cn } from "../../lib/utils";
import { useKeyboardShortcut } from "../../hooks/useKeyboardShortcut";
import UpdateButton from "../common/UpdateButton";
import Logo from "../../assets/favicon.svg";
import { api } from "../../lib/api";
import { useBreadcrumbs } from "../../hooks/useBreadcrumbs";
import TitleBar from "./TitleBar";
import SearchModal from "../search/SearchModal";
import TopHeaderSearch from "../search/SearchHeader";
import { useAuth } from "../../contexts/AuthContext";

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const isRTL = i18n.language === "ps";
  const [isDark, setIsDark] = useState(false);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const { logout } = useAuth();
  const breadcrumbs = useBreadcrumbs();

  const openSearch = () => setIsSearchOpen(true);
  const closeSearch = () => setIsSearchOpen(false);

  useKeyboardShortcut("k", openSearch, "ctrl");
  useKeyboardShortcut("k", openSearch, "meta");

  const handleLogout = async () => {
    await logout();
    toast.success(t("auth.logoutSuccess"));
  };

  // Theme functions
  const applyTheme = (dark: boolean) => {
    document.documentElement.classList.toggle("dark", dark);
    localStorage.setItem("theme", dark ? "dark" : "light");
  };

  const toggleTheme = () => {
    setIsDark(!isDark);
  };

  const navigation = [
    { name: t("nav.dashboard"), href: "/dashboard", icon: LayoutIcon, exact: true },
    { name: t("nav.patients"), href: "/patients", icon: PatientIcon },
    { name: t("nav.visits"), href: "/visits", icon: ClipboardListIcon },
    { name: t("nav.billings"), href: "/billing", icon: BillingIcon },
    { name: t("nav.expenses"), href: "/expenses", icon: CurrencyIcon, exact: true },
    { name: t("nav.reports"), href: "/reports", icon: ReportsIcon, exact: true },
    { name: t("nav.settings"), href: "/settings", icon: SettingsIcon, exact: true },
  ];

  const changeLanguage = (lng: "en" | "ps") => {
    i18n.changeLanguage(lng);
  };

  const goTo = useCallback((path: string) => {
    navigate(path);
  }, [navigate]);

  useKeyboardShortcut("n", () => goTo("/patients/new"), "ctrl");
  useKeyboardShortcut("b", () => goTo("/billing"), "ctrl");
  useKeyboardShortcut("r", () => goTo("/reports"), "ctrl");
  useKeyboardShortcut("d", () => goTo("/dashboard"), "ctrl");
  useKeyboardShortcut(",", () => goTo("/settings"), "ctrl");

  const { data: clinicSettings } = useQuery({
    queryKey: ["settings"],
    queryFn: api.settings.get,
    refetchInterval: false,
    staleTime: 30000,
  });

  const clinicName = clinicSettings?.clinic_name || t("dashboard.logo");
  const clinicLogo = clinicSettings?.clinic_logo || Logo;

  // Initialize isDark from localStorage or OS on mount
  useEffect(() => {
    const saved = localStorage.getItem("theme") as "dark" | "light" | null;
    if (saved) {
      setIsDark(saved === "dark");
    } else {
      // Respect OS preference if no saved value
      const prefersDark = window.matchMedia(
        "(prefers-color-scheme: dark)",
      ).matches;
      setIsDark(prefersDark);
    }
  }, []);

  // Apply theme whenever isDark changes
  useEffect(() => {
    applyTheme(isDark);
  }, [isDark]);

  return (
    <div className="flex flex-col h-screen w-screen bg-gray-50 dark:bg-gray-900">
      <TitleBar clinicLogo={clinicLogo} />
      <div className="flex flex-1 overflow-hidden">
      {/* Sidebar */}
      <aside
        className={cn(
          "flex flex-col border-r border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800 transition-all duration-300",
          sidebarOpen ? "w-64" : "w-16",
        )}
      >
        {/* Logo */}
        <div className="flex h-16 items-center justify-between border-b border-gray-200 px-4 dark:border-gray-700">
          {sidebarOpen && (
            <div className="flex items-center space-x-2">
              <img src={clinicLogo} alt="Clinic Logo" className="h-8 w-8 object-contain rounded" />
              <div>
                <span className="text-xl font-bold text-gray-900 dark:text-white truncate max-w-35 block">
                  {clinicName}
                </span>
              </div>
            </div>
          )}
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setSidebarOpen(!sidebarOpen)}
            className="cursor-pointer"
          >
            {sidebarOpen ? (
              <CollapseIcon
                className={cn("h-5 w-5", isRTL && "transform scale-x-[-1]")}
              />
            ) : (
              <ExpandIcon
                className={cn("h-5 w-5", isRTL && "transform scale-x-[-1]")}
              />
            )}
          </Button>
        </div>

        {/* Navigation */}
        <nav className="flex-1 space-y-1 p-2">
          {navigation.map((item) => {
            const isActive = item.exact
              ? location.pathname === item.href
              : location.pathname === item.href ||
                location.pathname.startsWith(`${item.href}/`);
            return (
              <NavLink
                key={item.href}
                to={item.href}
                title={sidebarOpen ? undefined : item.name}
                aria-label={item.name}
                className={cn(
                  "flex items-center rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                  isActive
                    ? "bg-primary text-white"
                    : "text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700",
                  !sidebarOpen && "justify-center",
                )}
              >
                <item.icon
                  className={cn(
                    "h-5 w-5 shrink-0",
                    isRTL && "transform scale-x-[-1]",
                  )}
                />
                {sidebarOpen && <span className="ms-3">{item.name}</span>}
              </NavLink>
            );
          })}
        </nav>

        {/* Language Switcher */}
        <div className="border-t border-gray-200 p-4 dark:border-gray-700">
          {sidebarOpen ? (
            <div className="space-y-2">
              <p className="text-xs font-medium text-gray-500 dark:text-gray-400">
                {t("settings.language")}
              </p>
              <div className="flex gap-1">
                {(["ps", "en"] as const).map((lng) => (
                  <Button
                    key={lng}
                    variant={i18n.language === lng ? "default" : "ghost"}
                    size="sm"
                    onClick={() => changeLanguage(lng)}
                    className="cursor-pointer"
                  >
                    {lng.toUpperCase()}
                  </Button>
                ))}
              </div>
            </div>
          ) : (
            <div className="flex justify-center">
              <Button
                variant="ghost"
                size="icon"
                onClick={() =>
                  changeLanguage(i18n.language === "ps" ? "en" : "ps")
                }
                className="cursor-pointer"
              >
                {i18n.language.toUpperCase()}
              </Button>
            </div>
          )}
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 overflow-auto">
        {/* Header */}
        <header className="sticky top-0 z-10 flex h-16 items-center justify-between border-b border-gray-200 bg-white px-6 dark:border-gray-700 dark:bg-gray-800">
          <div className="flex-1 min-w-0">
            <Breadcrumbs items={breadcrumbs} isRTL={isRTL} />
          </div>
          <div className="flex-1 flex items-center justify-end gap-2">
            {/* Global search */}
            <TopHeaderSearch
              onClick={openSearch}
              className="h-9 w-64 lg:w-80 text-xs"
            />

            {/* Theme toggle button */}
            <Button
              onClick={toggleTheme}
              variant="ghost"
              size="icon"
              className="cursor-pointer border rounded-lg dark:border-gray-500 hover:bg-muted/50 transition-colors"
              aria-label="Toggle dark mode"
            >
              {isDark ? <SunIcon size="lg" /> : <MoonIcon size="lg" />}
            </Button>

            {/* Update button */}
            <UpdateButton />
            {/* Logout button */}
            <Button
              onClick={handleLogout}
              variant="destructive"
              size="icon"
              className="cursor-pointer rounded-lg transition-colors"
              title={t("auth.logout")}
            >
              <svg
                className="h-5 w-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                />
              </svg>
            </Button>
          </div>
        </header>
        {/* Page content */}
        <div className="p-6">{children}</div>
      </main>
      </div>
      <SearchModal isOpen={isSearchOpen} onClose={closeSearch} />
    </div>
  );
};

export default MainLayout;
