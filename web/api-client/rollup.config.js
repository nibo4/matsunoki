import dts from 'rollup-plugin-dts'
import esbuild from 'rollup-plugin-esbuild'

export default [
  {
    input: `src/index.ts`,
    plugins: [esbuild({
      tsconfig: './tsconfig.json'
    })],
    output: [
      {
        file: `dist/index.cjs.js`,
        format: 'cjs',
        sourcemap: true,
      },
      {
        file: `dist/index.esm.js`,
        format: 'es',
        sourcemap: true,
      }
    ]
  },
  {
    input: `src/index.ts`,
    plugins: [dts()],
    output: {
      file: `dist/index.d.ts`,
      format: 'es',
    },
  }
]
