import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import svgPlugin from 'vite-plugin-solid-svg'

export default defineConfig({
  plugins: [solidPlugin(), svgPlugin()],
  server: {
    port: 3001
  },
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
