import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, '../shared'),
    },
  },
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://localhost:3000',
        ws: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          ag: ['ag-grid-community', 'ag-grid-react', 'ag-charts-community', 'ag-charts-react'],
          utils: ['lodash-es', 'date-fns', 'uuid'],
        },
      },
    },
  },
  define: {
    global: 'globalThis',
  },
  optimizeDeps: {
    include: [
      'react',
      'react-dom',
      'ag-grid-community',
      'ag-grid-react',
      'ag-charts-community',
      'ag-charts-react',
    ],
  },
})