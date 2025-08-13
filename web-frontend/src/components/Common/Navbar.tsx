import React from 'react'
import { Bell, Search, Menu, RefreshCw } from 'lucide-react'
import { useLocation } from 'react-router-dom'

interface NavbarProps {
  onMenuClick: () => void
}

export default function Navbar({ onMenuClick }: NavbarProps) {
  const location = useLocation()

  const getPageTitle = () => {
    switch (location.pathname) {
      case '/dashboard':
        return 'Dashboard'
      case '/data-sources':
        return 'Data Sources'
      case '/analytics':
        return 'Analytics'
      case '/settings':
        return 'Settings'
      default:
        return 'Dashboard'
    }
  }

  return (
    <div className="relative z-10 flex-shrink-0 flex h-16 bg-white shadow">
      {/* Mobile menu button */}
      <button
        type="button"
        className="px-4 border-r border-gray-200 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-primary-500 md:hidden"
        onClick={onMenuClick}
      >
        <span className="sr-only">Open sidebar</span>
        <Menu className="h-6 w-6" />
      </button>

      <div className="flex-1 px-4 flex justify-between">
        <div className="flex-1 flex items-center">
          {/* Page title */}
          <div className="flex items-center">
            <h1 className="text-xl font-semibold text-gray-900">
              {getPageTitle()}
            </h1>
          </div>

          {/* Search */}
          <div className="max-w-lg w-full lg:max-w-xs ml-6">
            <label htmlFor="search" className="sr-only">
              Search
            </label>
            <div className="relative">
              <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Search className="h-5 w-5 text-gray-400" />
              </div>
              <input
                id="search"
                name="search"
                className="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-primary-500 focus:border-primary-500 sm:text-sm"
                placeholder="Search dashboards, data sources..."
                type="search"
              />
            </div>
          </div>
        </div>

        <div className="ml-4 flex items-center md:ml-6 space-x-4">
          {/* Refresh button */}
          <button
            type="button"
            className="bg-white p-1 rounded-full text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
            title="Refresh data"
          >
            <span className="sr-only">Refresh data</span>
            <RefreshCw className="h-5 w-5" />
          </button>

          {/* Notifications */}
          <button
            type="button"
            className="bg-white p-1 rounded-full text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <span className="sr-only">View notifications</span>
            <Bell className="h-5 w-5" />
          </button>

          {/* Status indicator */}
          <div className="flex items-center space-x-2">
            <div className="h-2 w-2 bg-green-400 rounded-full"></div>
            <span className="text-sm text-gray-500">Connected</span>
          </div>

          {/* Profile dropdown */}
          <div className="relative">
            <div>
              <button
                type="button"
                className="max-w-xs bg-white flex items-center text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                id="user-menu-button"
                aria-expanded="false"
                aria-haspopup="true"
              >
                <span className="sr-only">Open user menu</span>
                <img
                  className="h-8 w-8 rounded-full"
                  src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
                  alt=""
                />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}