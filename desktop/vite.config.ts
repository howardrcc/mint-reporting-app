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
  
  // Tauri expects a fixed port and no HMR for production builds
  server: {
    port: 5173,
    strictPort: true,
    hmr: {
      port: 5174,
    },
  },
  
  // Use relative paths for assets in Tauri
  base: './',
  
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // Don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // Produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
    outDir: 'dist',
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          ag: ['ag-grid-community', 'ag-grid-react', 'ag-charts-community', 'ag-charts-react'],
        },
      },
    },
  },
  
  envPrefix: ['VITE_', 'TAURI_'],
  
  define: {
    global: 'globalThis',
  },
})