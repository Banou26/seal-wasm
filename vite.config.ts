import { defineConfig } from 'vite'

export default defineConfig((env) => ({
  build: {
    target: 'esnext',
    outDir: 'build',
    lib: {
      name: 'Stub',
      fileName: 'index',
      entry: 'src/index.ts',
      formats: ['es']
    }
  }
}))
