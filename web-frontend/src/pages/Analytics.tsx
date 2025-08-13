import React from 'react'
import { Play, Save, Download } from 'lucide-react'

export default function Analytics() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Analytics</h1>
          <p className="text-gray-600">Run custom queries and analysis</p>
        </div>
        
        <div className="flex items-center space-x-3">
          <button className="btn-outline">
            <Save className="h-4 w-4 mr-2" />
            Save Query
          </button>
          
          <button className="btn-primary">
            <Play className="h-4 w-4 mr-2" />
            Run Query
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <div className="card-header">
            <h2 className="text-lg font-medium text-gray-900">SQL Query</h2>
          </div>
          <div className="card-body">
            <textarea
              className="w-full h-64 font-mono text-sm border border-gray-300 rounded p-3"
              placeholder="SELECT * FROM your_table WHERE condition = 'value'"
            />
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <h2 className="text-lg font-medium text-gray-900">Query Results</h2>
          </div>
          <div className="card-body">
            <div className="text-center text-gray-500 py-12">
              Run a query to see results here
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}