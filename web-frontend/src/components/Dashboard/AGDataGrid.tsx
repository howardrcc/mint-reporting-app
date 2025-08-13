import React, { useMemo } from 'react'
import { AgGridReact } from 'ag-grid-react'
import { ColDef, GridReadyEvent, GridOptions } from 'ag-grid-community'
import { motion } from 'framer-motion'
import 'ag-grid-community/styles/ag-grid.css'
import 'ag-grid-community/styles/ag-theme-alpine.css'

interface GridConfig {
  title: string
  data?: any[]
  columns?: ColDef[]
  pagination?: boolean
  pageSize?: number
}

interface AGGridWidgetProps {
  title: string
  config: GridConfig
  onEdit?: () => void
  onDelete?: () => void
  isEditing?: boolean
}

// Sample data for demonstration
const generateSampleData = () => {
  return Array.from({ length: 100 }, (_, i) => ({
    id: i + 1,
    name: `Record ${i + 1}`,
    type: ['CSV', 'JSON', 'Parquet'][Math.floor(Math.random() * 3)],
    size: Math.floor(Math.random() * 1000000) + 1000,
    rows: Math.floor(Math.random() * 100000) + 1000,
    created: new Date(Date.now() - Math.random() * 10000000000).toLocaleDateString(),
    status: ['Active', 'Processing', 'Error'][Math.floor(Math.random() * 3)],
  }))
}

const defaultColumns: ColDef[] = [
  { 
    field: 'id', 
    headerName: 'ID', 
    width: 80,
    sortable: true,
    filter: 'agNumberColumnFilter'
  },
  { 
    field: 'name', 
    headerName: 'Name', 
    flex: 1,
    sortable: true,
    filter: 'agTextColumnFilter'
  },
  { 
    field: 'type', 
    headerName: 'Type', 
    width: 100,
    sortable: true,
    filter: 'agSetColumnFilter'
  },
  { 
    field: 'size', 
    headerName: 'Size (bytes)', 
    width: 120,
    sortable: true,
    filter: 'agNumberColumnFilter',
    valueFormatter: (params) => {
      const size = params.value
      if (size > 1000000) return `${(size / 1000000).toFixed(1)}MB`
      if (size > 1000) return `${(size / 1000).toFixed(1)}KB`
      return `${size}B`
    }
  },
  { 
    field: 'rows', 
    headerName: 'Rows', 
    width: 100,
    sortable: true,
    filter: 'agNumberColumnFilter',
    valueFormatter: (params) => new Intl.NumberFormat().format(params.value)
  },
  { 
    field: 'created', 
    headerName: 'Created', 
    width: 120,
    sortable: true,
    filter: 'agDateColumnFilter'
  },
  { 
    field: 'status', 
    headerName: 'Status', 
    width: 100,
    sortable: true,
    filter: 'agSetColumnFilter',
    cellRenderer: (params: any) => {
      const status = params.value
      const colorClass = {
        'Active': 'bg-green-100 text-green-800',
        'Processing': 'bg-yellow-100 text-yellow-800',
        'Error': 'bg-red-100 text-red-800'
      }[status] || 'bg-gray-100 text-gray-800'
      
      return `<span class="px-2 py-1 text-xs font-medium rounded-full ${colorClass}">${status}</span>`
    }
  },
]

export default function AGGridWidget({ 
  title, 
  config, 
  onEdit, 
  onDelete, 
  isEditing 
}: AGGridWidgetProps) {
  const rowData = useMemo(() => config.data || generateSampleData(), [config.data])
  const columnDefs = useMemo(() => config.columns || defaultColumns, [config.columns])

  const gridOptions: GridOptions = {
    defaultColDef: {
      resizable: true,
      sortable: true,
      filter: true,
    },
    pagination: config.pagination !== false,
    paginationPageSize: config.pageSize || 20,
    rowSelection: 'multiple',
    enableRangeSelection: true,
    enableCharts: true,
    sideBar: true,
    suppressRowClickSelection: false,
    suppressCellFocus: false,
    enableCellTextSelection: true,
    animateRows: true,
  }

  const onGridReady = (params: GridReadyEvent) => {
    params.api.sizeColumnsToFit()
  }

  return (
    <motion.div
      className="card h-full flex flex-col"
      whileHover={{ scale: isEditing ? 1 : 1.005 }}
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
      
      <div className="flex-1 min-h-0">
        <div className="ag-theme-alpine h-full w-full">
          <AgGridReact
            rowData={rowData}
            columnDefs={columnDefs}
            gridOptions={gridOptions}
            onGridReady={onGridReady}
          />
        </div>
      </div>
    </motion.div>
  )
}