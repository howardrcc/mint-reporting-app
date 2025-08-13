import React from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'
import { ErrorBoundary } from 'react-error-boundary'

import Layout from './components/Common/Layout'
import Dashboard from './pages/Dashboard'
import DataSources from './pages/DataSources'
import Analytics from './pages/Analytics'
import Settings from './pages/Settings'
import ErrorFallback from './components/Common/ErrorFallback'
import LoadingSpinner from './components/Common/LoadingSpinner'

// Lazy load components for better performance
const LazyDashboard = React.lazy(() => import('./pages/Dashboard'))
const LazyDataSources = React.lazy(() => import('./pages/DataSources'))
const LazyAnalytics = React.lazy(() => import('./pages/Analytics'))
const LazySettings = React.lazy(() => import('./pages/Settings'))

function App() {
  return (
    <ErrorBoundary FallbackComponent={ErrorFallback}>
      <Layout>
        <React.Suspense fallback={<LoadingSpinner />}>
          <Routes>
            {/* Default route redirects to dashboard */}
            <Route path="/" element={<Navigate to="/dashboard" replace />} />
            
            {/* Main application routes */}
            <Route path="/dashboard" element={<LazyDashboard />} />
            <Route path="/dashboard/:id" element={<LazyDashboard />} />
            <Route path="/data-sources" element={<LazyDataSources />} />
            <Route path="/analytics" element={<LazyAnalytics />} />
            <Route path="/settings" element={<LazySettings />} />
            
            {/* Catch-all route for 404 */}
            <Route path="*" element={<Navigate to="/dashboard" replace />} />
          </Routes>
        </React.Suspense>
      </Layout>
    </ErrorBoundary>
  )
}

export default App