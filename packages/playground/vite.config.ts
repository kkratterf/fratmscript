import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'
import agentFiles from './agent-files'
import sitePages from './site-pages'

export default defineConfig({
  plugins: [react(), tailwindcss(), agentFiles(), sitePages()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      // Shared with the VS Code extension's language server
      '@keywords': path.resolve(__dirname, '../vscode-extension/src/server/keywords.ts'),
    },
  },
  base: '/',
  // ES workers can split chunks, so sql.js loads only when a program needs it
  worker: { format: 'es' },
  // The example picker imports ../../examples/*.fratm
  server: { fs: { allow: [path.resolve(__dirname, '../..')] } },
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
