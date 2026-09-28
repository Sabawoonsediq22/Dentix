import { lazy, Suspense, useEffect, useState } from "react";
import { Routes, Route, Navigate } from "react-router-dom";
import { useTranslation } from "react-i18next";
import MainLayout from "./components/layouts/MainLayout";
import "./i18n";
import { LoadingSpinner } from "./components/ui";
import { ErrorBoundary } from "./components/common/ErrorBoundary";
import { useAuth } from "./contexts/AuthContext";
import Login from "./pages/Login";
import Setup from "./pages/Setup";

const Dashboard = lazy(() => import("./pages/Dashboard"));
const Patients = lazy(() => import("./pages/Patients"));
const NewPatient = lazy(() => import("./pages/NewPatient"));
const NewVisit = lazy(() => import("./pages/NewVisit"));
const PatientProfile = lazy(() => import("./pages/PatientProfile"));
const Reports = lazy(() => import("./pages/Reports"));
const Settings = lazy(() => import("./pages/Settings"));
const Billing = lazy(() => import("./pages/Billing"));
const Expenses = lazy(() => import("./pages/Expenses"));
const AllVisits = lazy(() => import("./pages/AllVisits"));
const NotFound = lazy(() => import("./pages/NotFound"));

const PageLoader = () => (
  <div className="flex h-[60vh] items-center justify-center">
    <LoadingSpinner size="lg" />
  </div>
);

function App() {
  const { i18n } = useTranslation();
  const { isAuthenticated, isLoading, hasUsers } = useAuth();
  const [appReady, setAppReady] = useState(false);

  useEffect(() => {
    document.documentElement.dir = ["ps"].includes(i18n.language)
      ? "rtl"
      : "ltr";
    document.documentElement.lang = i18n.language;
    setAppReady(true);
  }, [i18n.language]);

  if (isLoading || !appReady) {
    return (
      <div className="flex h-screen items-center justify-center">
        <LoadingSpinner size="lg" />
      </div>
    );
  }

  if (!hasUsers) {
    return (
      <ErrorBoundary>
        <Setup />
      </ErrorBoundary>
    );
  }

  if (!isAuthenticated) {
    return (
      <ErrorBoundary>
        <Login />
      </ErrorBoundary>
    );
  }

  return (
    <ErrorBoundary>
      <MainLayout>
        <Suspense fallback={<PageLoader />}>
          <Routes>
            <Route path="/" element={<Navigate to="/dashboard" replace />} />
            <Route path="/dashboard" element={<Dashboard />} />
            <Route path="/patients" element={<Patients />} />
            <Route path="/patients/new" element={<NewPatient />} />
            <Route path="/patients/:id/visits/new" element={<NewVisit />} />
            <Route path="/patients/:id" element={<PatientProfile />} />
            <Route path="/billing" element={<Billing />} />
            <Route path="/billing/invoices/:id" element={<Billing />} />
            <Route path="/billing/receipts/:id" element={<Billing />} />
            <Route path="/billing/payments/:id" element={<Billing />} />
            <Route path="/expenses" element={<Expenses />} />
            <Route path="/visits" element={<AllVisits />} />
            <Route path="/visits/:id" element={<Patients />} />
            <Route path="/treatments/:id" element={<Patients />} />
            <Route path="/reports" element={<Reports />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </Suspense>
      </MainLayout>
    </ErrorBoundary>
  );
}

export default App;
