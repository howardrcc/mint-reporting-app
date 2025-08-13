import React from 'react'
import { Plus, Upload, Database, FileText } from 'lucide-react'

export default function DataSources() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Data Sources</h1>
          <p className="text-gray-600">Manage your data connections and uploads</p>
        </div>
        
        <div className="flex items-center space-x-3">
          <button className="btn-outline">
            <Database className="h-4 w-4 mr-2" />
            Connect Database
          </button>
          
          <button className="btn-primary">
            <Upload className="h-4 w-4 mr-2" />
            Upload File
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* Placeholder cards */}
        {[1, 2, 3].map((i) => (
          <div key={i} className="card">
            <div className="card-body">
              <div className="flex items-center">
                <FileText className="h-8 w-8 text-primary-600 mr-3" />
                <div>
                  <h3 className="text-lg font-medium text-gray-900">Sample Data {i}</h3>
                  <p className="text-sm text-gray-600">CSV • 1.2MB • 15,000 rows</p>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}