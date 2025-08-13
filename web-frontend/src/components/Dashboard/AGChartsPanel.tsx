import React, { useEffect, useRef } from 'react'
import { AgCharts } from 'ag-charts-react'
import { AgChartOptions } from 'ag-charts-community'
import { motion } from 'framer-motion'

interface ChartConfig {
  type: 'line' | 'bar' | 'column' | 'area' | 'pie' | 'donut' | 'scatter'
  title: string
  data?: any[]
  xKey?: string
  yKey?: string
  theme?: string
}

interface AGChartWidgetProps {
  title: string
  config: ChartConfig
  onEdit?: () => void
  onDelete?: () => void
  isEditing?: boolean
}

// Sample data for demonstration
const generateSampleData = (type: string) => {
  switch (type) {
    case 'line':
    case 'area':
      return [
        { month: 'Jan', value: 120 },
        { month: 'Feb', value: 150 },
        { month: 'Mar', value: 180 },
        { month: 'Apr', value: 140 },
        { month: 'May', value: 200 },
        { month: 'Jun', value: 240 },
        { month: 'Jul', value: 220 },
        { month: 'Aug', value: 260 },
        { month: 'Sep', value: 290 },
        { month: 'Oct', value: 280 },
        { month: 'Nov', value: 320 },
        { month: 'Dec', value: 350 },
      ]
    case 'bar':
    case 'column':
      return [
        { category: 'CSV', count: 45 },
        { category: 'JSON', count: 32 },
        { category: 'Parquet', count: 28 },
        { category: 'Excel', count: 15 },
        { category: 'Database', count: 38 },
      ]
    case 'pie':
    case 'donut':
      return [
        { label: 'Active', value: 65, color: '#3b82f6' },
        { label: 'Inactive', value: 25, color: '#94a3b8' },
        { label: 'Pending', value: 10, color: '#f59e0b' },
      ]
    default:
      return []
  }
}

export default function AGChartWidget({ 
  title, 
  config, 
  onEdit, 
  onDelete, 
  isEditing 
}: AGChartWidgetProps) {
  const data = config.data || generateSampleData(config.type)
  
  const chartOptions: AgChartOptions = {
    data,
    theme: config.theme || 'ag-default',
    title: {
      text: config.title,
    },
    background: {
      fill: 'white',
    },
    padding: {
      top: 20,
      right: 20,
      bottom: 20,
      left: 20,
    },
  }

  // Configure chart based on type
  switch (config.type) {
    case 'line':
      Object.assign(chartOptions, {
        series: [{
          type: 'line',
          xKey: 'month',
          yKey: 'value',
          stroke: '#3b82f6',
          marker: {
            enabled: true,
            fill: '#3b82f6',
          },
        }],
        axes: [
          {
            type: 'category',
            position: 'bottom',
          },
          {
            type: 'number',
            position: 'left',
          },
        ],
      })
      break

    case 'area':
      Object.assign(chartOptions, {
        series: [{
          type: 'area',
          xKey: 'month',
          yKey: 'value',
          fill: '#3b82f6',
          fillOpacity: 0.7,
          stroke: '#1d4ed8',
        }],
        axes: [
          {
            type: 'category',
            position: 'bottom',
          },
          {
            type: 'number',
            position: 'left',
          },
        ],
      })
      break

    case 'bar':
      Object.assign(chartOptions, {
        series: [{
          type: 'bar',
          xKey: 'category',
          yKey: 'count',
          fill: '#3b82f6',
        }],
        axes: [
          {
            type: 'category',
            position: 'left',
          },
          {
            type: 'number',
            position: 'bottom',
          },
        ],
      })
      break

    case 'column':
      Object.assign(chartOptions, {
        series: [{
          type: 'column',
          xKey: 'category',
          yKey: 'count',
          fill: '#3b82f6',
        }],
        axes: [
          {
            type: 'category',
            position: 'bottom',
          },
          {
            type: 'number',
            position: 'left',
          },
        ],
      })
      break

    case 'pie':
      Object.assign(chartOptions, {
        series: [{
          type: 'pie',
          angleKey: 'value',
          labelKey: 'label',
          innerRadiusRatio: 0,
        }],
      })
      break

    case 'donut':
      Object.assign(chartOptions, {
        series: [{
          type: 'pie',
          angleKey: 'value',
          labelKey: 'label',
          innerRadiusRatio: 0.6,
        }],
      })
      break
  }

  return (
    <motion.div
      className="card h-full"
      whileHover={{ scale: isEditing ? 1 : 1.01 }}
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
        <div className="chart-container h-full">
          <AgCharts options={chartOptions} />
        </div>
      </div>
    </motion.div>
  )
}