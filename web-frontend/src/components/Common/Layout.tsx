import React, { useState } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { 
  BarChart3, 
  Database, 
  Settings, 
  Menu,
  X,
  Home,
  TrendingUp
} from 'lucide-react'
import { motion, AnimatePresence } from 'framer-motion'
import clsx from 'clsx'

import Navbar from './Navbar'

interface LayoutProps {
  children: React.ReactNode
}

const navigation = [
  { name: 'Dashboard', href: '/dashboard', icon: Home },
  { name: 'Data Sources', href: '/data-sources', icon: Database },
  { name: 'Analytics', href: '/analytics', icon: TrendingUp },
  { name: 'Charts', href: '/charts', icon: BarChart3 },
  { name: 'Settings', href: '/settings', icon: Settings },
]

export default function Layout({ children }: LayoutProps) {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const location = useLocation()

  return (
    <div className="h-screen flex overflow-hidden bg-gray-100">
      {/* Mobile sidebar overlay */}
      <AnimatePresence>
        {sidebarOpen && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 flex z-40 md:hidden"
            onClick={() => setSidebarOpen(false)}
          >
            <div className="fixed inset-0 bg-gray-600 bg-opacity-75" />
            <motion.div
              initial={{ x: -240 }}
              animate={{ x: 0 }}
              exit={{ x: -240 }}
              className="relative flex-1 flex flex-col max-w-xs w-full bg-white"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="absolute top-0 right-0 -mr-12 pt-2">
                <button
                  type="button"
                  className="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                  onClick={() => setSidebarOpen(false)}
                >
                  <span className="sr-only">Close sidebar</span>
                  <X className="h-6 w-6 text-white" />
                </button>
              </div>
              <Sidebar />
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Desktop sidebar */}
      <div className="hidden md:flex md:flex-shrink-0">
        <div className="flex flex-col w-64">
          <Sidebar />
        </div>
      </div>

      {/* Main content */}
      <div className="flex flex-col w-0 flex-1 overflow-hidden">
        <Navbar onMenuClick={() => setSidebarOpen(true)} />
        
        <main className="flex-1 relative overflow-y-auto focus:outline-none">
          <div className="py-6">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
              {children}
            </div>
          </div>
        </main>
      </div>
    </div>
  )
}

function Sidebar() {
  const location = useLocation()

  return (
    <div className="flex flex-col h-0 flex-1 border-r border-gray-200 bg-white">
      {/* Logo */}
      <div className="flex items-center h-16 flex-shrink-0 px-4 bg-primary-600">
        <div className="flex items-center">
          <div className="flex-shrink-0">
            <BarChart3 className="h-8 w-8 text-white" />
          </div>
          <div className="ml-3">
            <h1 className="text-lg font-semibold text-white">DuckDB Dashboard</h1>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-2 py-4 bg-white space-y-1 overflow-y-auto">
        {navigation.map((item) => {
          const isActive = location.pathname === item.href || 
            (item.href !== '/dashboard' && location.pathname.startsWith(item.href))
          
          return (
            <Link
              key={item.name}
              to={item.href}
              className={clsx(
                'group flex items-center px-2 py-2 text-sm font-medium rounded-md transition-colors duration-200',
                isActive
                  ? 'bg-primary-100 text-primary-900'
                  : 'text-gray-600 hover:bg-gray-50 hover:text-gray-900'
              )}
            >
              <item.icon
                className={clsx(
                  'mr-3 flex-shrink-0 h-5 w-5',
                  isActive ? 'text-primary-500' : 'text-gray-400 group-hover:text-gray-500'
                )}
              />
              {item.name}
            </Link>
          )
        })}
      </nav>

      {/* Bottom section */}
      <div className="flex-shrink-0 flex border-t border-gray-200 p-4">
        <div className="flex items-center">
          <div>
            <img
              className="inline-block h-9 w-9 rounded-full"
              src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
              alt=""
            />
          </div>
          <div className="ml-3">
            <p className="text-xs font-medium text-gray-700 group-hover:text-gray-900">
              Demo User
            </p>
            <p className="text-xs font-medium text-gray-500 group-hover:text-gray-700">
              View profile
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}