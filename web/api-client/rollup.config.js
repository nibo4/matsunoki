import dts from 'rollup-plugin-dts'
import esbuild from 'rollup-plugin-esbuild'

export default [
  {
    input: `src/index.js`,
    plugins: [esbuild({
      tsconfig: './tsconfig.json'
    })],
    output: [
      {
        file: `dist/bundle.js`,
        format: 'cjs',
        sourcemap: true,
      },
      {
        file: `dist/bundle.js`,
        format: 'es',
        sourcemap: true,
      }
    ]
  },
  {
    input: `src/index.js`,
    plugins: [dts()],
    output: {
      file: `dist/bundle.d.ts`,
      format: 'es',
    },
  }
]
