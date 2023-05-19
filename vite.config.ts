import { defineConfig } from 'vite'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import typescript2 from 'rollup-plugin-typescript2'

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
  },
  plugins: [
    wasm(),
    topLevelAwait()
  ]
}))
