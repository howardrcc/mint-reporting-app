import React, { useState } from 'react'
import { motion } from 'framer-motion'

interface FilterConfig {
  title: string
  field: string
  type: 'text' | 'select' | 'date' | 'number'
  options?: Array<{ value: any; label: string }>
  defaultValue?: any
}

interface FilterWidgetProps {
  title: string
  config: FilterConfig
  onEdit?: () => void
  onDelete?: () => void
  isEditing?: boolean
}

export default function FilterWidget({ 
  title, 
  config, 
  onEdit, 
  onDelete, 
  isEditing 
}: FilterWidgetProps) {
  const [value, setValue] = useState(config.defaultValue || '')

  const renderFilter = () => {
    switch (config.type) {
      case 'text':
        return (
          <input
            type="text"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            placeholder={`Filter by ${config.field}`}
            className="input"
          />
        )
      
      case 'select':
        return (
          <select
            value={value}
            onChange={(e) => setValue(e.target.value)}
            className="input"
          >
            <option value="">All {config.field}</option>
            {config.options?.map((option) => (
              <option key={option.value} value={option.value}>
                {option.label}
              </option>
            ))}
          </select>
        )
      
      case 'date':
        return (
          <input
            type="date"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            className="input"
          />
        )
      
      case 'number':
        return (
          <input
            type="number"
            value={value}
            onChange={(e) => setValue(e.target.value)}
            placeholder={`Filter by ${config.field}`}
            className="input"
          />
        )
      
      default:
        return null
    }
  }

  return (
    <motion.div
      className="card h-full"
      whileHover={{ scale: isEditing ? 1 : 1.02 }}
      transition={{ duration: 0.2 }}
    >
      {isEditing && (
        <div className="widget-header">
          <span className="widget-title">{title}</span>
          <div className="widget-actions">
            <button onClick={onEdit} className="btn-sm btn-outline">
              Edit
            </button>
            <button onClick={onDelete} className="btn-sm btn-danger">
              Delete
            </button>
          </div>
        </div>
      )}
      
      <div className="widget-content">
        <div className="filter-widget">
          <label className="filter-label">
            {config.title}
          </label>
          {renderFilter()}
        </div>
      </div>
    </motion.div>
  )
}