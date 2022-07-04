import dts from 'rollup-plugin-dts'
import css from 'rollup-plugin-import-css'

export default [
  {
    input: `src/index.tsx`,
    plugins: [dts(), css()],
    output: {
      file: `lib/index.d.ts`,
      format: 'es',
    },
  }
]
