import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import globPlugin from 'vite-plugin-glob'
import svgPlugin from 'vite-plugin-solid-svg'

export default defineConfig({
  plugins: [globPlugin(), solidPlugin(), svgPlugin()],
  server: {
    port: 3001
  },
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
