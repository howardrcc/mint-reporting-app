import React from 'react'
import { Save, RefreshCw } from 'lucide-react'

export default function Settings() {
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
          <p className="text-gray-600">Configure your dashboard preferences</p>
        </div>
        
        <div className="flex items-center space-x-3">
          <button className="btn-outline">
            <RefreshCw className="h-4 w-4 mr-2" />
            Reset
          </button>
          
          <button className="btn-primary">
            <Save className="h-4 w-4 mr-2" />
            Save Changes
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <div className="card-header">
            <h2 className="text-lg font-medium text-gray-900">General</h2>
          </div>
          <div className="card-body space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Theme
              </label>
              <select className="input">
                <option>Light</option>
                <option>Dark</option>
                <option>Auto</option>
              </select>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Language
              </label>
              <select className="input">
                <option>English</option>
                <option>Spanish</option>
                <option>French</option>
              </select>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <h2 className="text-lg font-medium text-gray-900">Performance</h2>
          </div>
          <div className="card-body space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Refresh Interval (seconds)
              </label>
              <input type="number" className="input" defaultValue="30" />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Max Rows per Page
              </label>
              <input type="number" className="input" defaultValue="100" />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}