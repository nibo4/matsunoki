import path from 'path'
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import svgPlugin from 'vite-plugin-solid-svg'

export default defineConfig(({ command, mode }) =>{
  return {
    plugins: [solidPlugin(), svgPlugin()],
    build: {
      watch: mode === 'development' ? {
        include: ["./src/**/*", "./src/*"],
        exclude: ["./src/App.tsx"]
      } : null,
      lib: {
        entry: path.resolve(__dirname, 'src/index.tsx'),
        name: 'matsunoki-ui',
        fileName: 'matsunoki-ui'
      },
      rollupOptions: {
        external: ['solid-js','solid-app-router'],
        output: {
          globals: {
            "solid-js": 'SolidJS',
            'solid-app-router': "SolidJSAppRouter"
          }
        }
      },
      target: 'esnext',
      polyfillDynamicImport: false,
    },
  }
});
