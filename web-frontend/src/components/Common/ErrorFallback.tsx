import React from 'react'
import { AlertTriangle, RefreshCw } from 'lucide-react'

interface ErrorFallbackProps {
  error: Error
  resetErrorBoundary: () => void
}

export default function ErrorFallback({ error, resetErrorBoundary }: ErrorFallbackProps) {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full bg-white shadow-lg rounded-lg p-6">
        <div className="flex items-center mb-4">
          <AlertTriangle className="h-8 w-8 text-red-500 mr-3" />
          <h1 className="text-xl font-semibold text-gray-900">Something went wrong</h1>
        </div>
        
        <p className="text-gray-600 mb-6">
          We're sorry, but something unexpected happened. Please try refreshing the page.
        </p>
        
        <details className="mb-6">
          <summary className="cursor-pointer text-sm text-gray-500 mb-2">
            Error details
          </summary>
          <pre className="text-xs bg-gray-100 p-3 rounded overflow-auto max-h-32">
            {error.message}
          </pre>
        </details>
        
        <div className="flex space-x-3">
          <button
            onClick={resetErrorBoundary}
            className="btn-primary flex items-center"
          >
            <RefreshCw className="h-4 w-4 mr-2" />
            Try again
          </button>
          
          <button
            onClick={() => window.location.reload()}
            className="btn-outline"
          >
            Refresh page
          </button>
        </div>
      </div>
    </div>
  )
}