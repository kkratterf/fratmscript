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
  },
})
