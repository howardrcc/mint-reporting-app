import React, { useState, useEffect } from 'react'
import { Plus, Settings, Download, Share } from 'lucide-react'
import { Responsive, WidthProvider } from 'react-grid-layout'
import 'react-grid-layout/css/styles.css'
import 'react-resizable/css/styles.css'

import AGChartWidget from '../components/Dashboard/AGChartsPanel'
import AGGridWidget from '../components/Dashboard/AGDataGrid'
import MetricWidget from '../components/Dashboard/MetricsCards'
import FilterWidget from '../components/Dashboard/FilterPanel'

const ResponsiveGridLayout = WidthProvider(Responsive)

interface DashboardWidget {
  i: string
  x: number
  y: number
  w: number
  h: number
  type: 'chart' | 'grid' | 'metric' | 'filter'
  config: any
}

const defaultLayouts = {
  lg: [
    { i: 'metric-1', x: 0, y: 0, w: 3, h: 2, type: 'metric' as const, config: { title: 'Total Records', value: '1.2M', trend: { direction: 'up', percentage: 12.5 } } },
    { i: 'metric-2', x: 3, y: 0, w: 3, h: 2, type: 'metric' as const, config: { title: 'Data Sources', value: '23', trend: { direction: 'up', percentage: 8.3 } } },
    { i: 'metric-3', x: 6, y: 0, w: 3, h: 2, type: 'metric' as const, config: { title: 'Queries/Hour', value: '847', trend: { direction: 'down', percentage: 2.1 } } },
    { i: 'metric-4', x: 9, y: 0, w: 3, h: 2, type: 'metric' as const, config: { title: 'Active Users', value: '156', trend: { direction: 'up', percentage: 15.7 } } },
    { i: 'chart-1', x: 0, y: 2, w: 6, h: 4, type: 'chart' as const, config: { type: 'line', title: 'Data Trends' } },
    { i: 'chart-2', x: 6, y: 2, w: 6, h: 4, type: 'chart' as const, config: { type: 'bar', title: 'Source Distribution' } },
    { i: 'grid-1', x: 0, y: 6, w: 12, h: 5, type: 'grid' as const, config: { title: 'Recent Data' } },
  ]
}

export default function Dashboard() {
  const [layouts, setLayouts] = useState(defaultLayouts)
  const [widgets, setWidgets] = useState<DashboardWidget[]>(defaultLayouts.lg)
  const [isEditing, setIsEditing] = useState(false)

  const onLayoutChange = (layout: any, layouts: any) => {
    setLayouts(layouts)
    // Update widgets with new positions
    const updatedWidgets = widgets.map(widget => {
      const layoutItem = layout.find((l: any) => l.i === widget.i)
      return layoutItem ? { ...widget, ...layoutItem } : widget
    })
    setWidgets(updatedWidgets)
  }

  const renderWidget = (widget: DashboardWidget) => {
    const baseProps = {
      key: widget.i,
      title: widget.config.title || `Widget ${widget.i}`,
      onEdit: () => console.log('Edit widget', widget.i),
      onDelete: () => console.log('Delete widget', widget.i),
      isEditing,
    }

    switch (widget.type) {
      case 'metric':
        return <MetricWidget {...baseProps} config={widget.config} />
      case 'chart':
        return <AGChartWidget {...baseProps} config={widget.config} />
      case 'grid':
        return <AGGridWidget {...baseProps} config={widget.config} />
      case 'filter':
        return <FilterWidget {...baseProps} config={widget.config} />
      default:
        return <div key={widget.i} className="widget-content">Unknown widget type</div>
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Dashboard Overview</h1>
          <p className="text-gray-600">Monitor your data analytics in real-time</p>
        </div>
        
        <div className="flex items-center space-x-3">
          <button
            onClick={() => setIsEditing(!isEditing)}
            className={`btn ${isEditing ? 'btn-primary' : 'btn-outline'}`}
          >
            <Settings className="h-4 w-4 mr-2" />
            {isEditing ? 'Done Editing' : 'Edit Dashboard'}
          </button>
          
          <button className="btn-outline">
            <Download className="h-4 w-4 mr-2" />
            Export
          </button>
          
          <button className="btn-outline">
            <Share className="h-4 w-4 mr-2" />
            Share
          </button>
          
          <button className="btn-primary">
            <Plus className="h-4 w-4 mr-2" />
            Add Widget
          </button>
        </div>
      </div>

      {/* Dashboard Grid */}
      <div className="dashboard-grid">
        <ResponsiveGridLayout
          className="layout"
          layouts={layouts}
          onLayoutChange={onLayoutChange}
          breakpoints={{ lg: 1200, md: 996, sm: 768, xs: 480, xxs: 0 }}
          cols={{ lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 }}
          rowHeight={60}
          isDraggable={isEditing}
          isResizable={isEditing}
          margin={[16, 16]}
          useCSSTransforms={true}
        >
          {widgets.map(renderWidget)}
        </ResponsiveGridLayout>
      </div>
    </div>
  )
}