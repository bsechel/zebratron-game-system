import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    fs: {
      allow: ['..']
    }
  },
  build: {
    lib: {
      entry: 'src/index.ts',
      name: 'ZebratronGameSystem',
      fileName: 'zebratron-game-system'
    },
    rollupOptions: {
      external: [],
      output: {
        globals: {}
      }
    }
  }
})