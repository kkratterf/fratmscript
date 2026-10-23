import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      // Shared with the VS Code extension's language server
      '@keywords': path.resolve(__dirname, '../vscode-extension/src/server/keywords.ts'),
    },
  },
  base: '/fratmscript/',
  build: {
    outDir: 'dist',
    rollupOptions: {
      output: {
        // Vendor code changes less often than the app: separate chunks
        // cache better across deploys and stay under Vite's size warning.
        manualChunks(id) {
          if (!id.includes('node_modules')) return
          if (/[\\/](@codemirror|@lezer|style-mod|w3c-keyname|crelt)[\\/]/.test(id)) return 'codemirror'
          if (/node_modules[\\/](react|react-dom|scheduler)[\\/]/.test(id)) return 'react'
          return 'vendor'
        },
      },
    },
  },
})
