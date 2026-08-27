import { defineConfig } from 'vite'

export default defineConfig({
  base: '/zgs-static/',
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