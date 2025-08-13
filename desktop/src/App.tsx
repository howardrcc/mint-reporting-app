// Desktop version uses the same React app as web frontend
// but with Tauri-specific integrations

import React from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'
import { ErrorBoundary } from 'react-error-boundary'

import Layout from '../../web-frontend/src/components/Common/Layout'
import ErrorFallback from '../../web-frontend/src/components/Common/ErrorFallback'
import LoadingSpinner from '../../web-frontend/src/components/Common/LoadingSpinner'

// Import pages from web frontend
import Dashboard from '../../web-frontend/src/pages/Dashboard'
import DataSources from '../../web-frontend/src/pages/DataSources'
import Analytics from '../../web-frontend/src/pages/Analytics'
import Settings from '../../web-frontend/src/pages/Settings'

// Lazy load components for better performance
const LazyDashboard = React.lazy(() => import('../../web-frontend/src/pages/Dashboard'))
const LazyDataSources = React.lazy(() => import('../../web-frontend/src/pages/DataSources'))
const LazyAnalytics = React.lazy(() => import('../../web-frontend/src/pages/Analytics'))
const LazySettings = React.lazy(() => import('../../web-frontend/src/pages/Settings'))

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