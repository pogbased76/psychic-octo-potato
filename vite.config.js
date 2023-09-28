// vite.config.js
import { defineConfig } from 'vite'

export default defineConfig({
  root: 'dist', // Serve files from the 'dist' directory
  build: {
    outDir: 'dist', // Build output directory
  },
  server: {
    open: true, // Open browser on server start
  },
})
