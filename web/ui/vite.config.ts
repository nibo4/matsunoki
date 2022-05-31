import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import GlobPlugin from 'vite-plugin-glob'

export default defineConfig({
  plugins: [GlobPlugin(), solidPlugin()],
  server: {
    port: 3001
  },
  build: {
    target: 'esnext',
    polyfillDynamicImport: false,
  },
});
