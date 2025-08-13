import React from 'react'
import { TrendingUp, TrendingDown, Minus } from 'lucide-react'
import { motion } from 'framer-motion'
import clsx from 'clsx'

interface MetricConfig {
  title: string
  value: string | number
  format?: 'number' | 'currency' | 'percentage'
  trend?: {
    direction: 'up' | 'down' | 'neutral'
    percentage: number
    period?: string
  }
  icon?: React.ComponentType<any>
  color?: string
}

interface MetricWidgetProps {
  title: string
  config: MetricConfig
  onEdit?: () => void
  onDelete?: () => void
  isEditing?: boolean
}

export default function MetricWidget({ 
  title, 
  config, 
  onEdit, 
  onDelete, 
  isEditing 
}: MetricWidgetProps) {
  const formatValue = (value: string | number, format?: string) => {
    if (typeof value === 'string') return value
    
    switch (format) {
      case 'currency':
        return new Intl.NumberFormat('en-US', { 
          style: 'currency', 
          currency: 'USD' 
        }).format(value)
      case 'percentage':
        return `${value}%`
      case 'number':
        return new Intl.NumberFormat('en-US').format(value)
      default:
        return value.toString()
    }
  }

  const getTrendIcon = (direction: string) => {
    switch (direction) {
      case 'up':
        return TrendingUp
      case 'down':
        return TrendingDown
      default:
        return Minus
    }
  }

  const getTrendColor = (direction: string) => {
    switch (direction) {
      case 'up':
        return 'text-green-600'
      case 'down':
        return 'text-red-600'
      default:
        return 'text-gray-600'
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
        <div className="metric-widget">
          {/* Icon */}
          {config.icon && (
            <div className="mb-2">
              <config.icon className="h-8 w-8 text-primary-600" />
            </div>
          )}

          {/* Value */}
          <div className="metric-value" style={{ color: config.color }}>
            {formatValue(config.value, config.format)}
          </div>

          {/* Title */}
          <div className="metric-label">
            {config.title}
          </div>

          {/* Trend */}
          {config.trend && (
            <div className={clsx('metric-trend', getTrendColor(config.trend.direction))}>
              {React.createElement(getTrendIcon(config.trend.direction), {
                className: 'h-3 w-3 mr-1'
              })}
              <span className="font-medium">
                {config.trend.percentage > 0 ? '+' : ''}{config.trend.percentage}%
              </span>
              {config.trend.period && (
                <span className="ml-1 text-gray-500">
                  vs {config.trend.period}
                </span>
              )}
            </div>
          )}
        </div>
      </div>
    </motion.div>
  )
}