import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import svgPlugin from 'vite-plugin-solid-svg'
import ssrPlugin from 'vite-plugin-ssr/plugin'

export default defineConfig({
  plugins: [solidPlugin(), svgPlugin(), ssrPlugin()],
  server: {
    port: 3001
  },
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
