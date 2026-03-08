import { defineConfig } from 'vite'

export default defineConfig({
  base: '/zgs/',
  server: {
    fs: {
      allow: ['..']
    }
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true
  }
})