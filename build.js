
require('esbuild').build({
  watch: process.argv.includes('-w') || process.argv.includes('--watch'),
  entryPoints: ['./src/index.ts'],
  format: 'esm',
  bundle: true,
  outfile: './build/index.js',
  publicPath: '/',
  plugins: [require('esbuild-plugin-wasm').default()],
}).catch(() => process.exit(1))
