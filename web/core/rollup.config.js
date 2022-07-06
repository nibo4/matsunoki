import dts from 'rollup-plugin-dts'
import esbuild from 'rollup-plugin-esbuild'
import metaEnv from 'rollup-plugin-import-meta-env'

export default [
  {
    input: `src/index.ts`,
    plugins: [esbuild({
      tsconfig: './tsconfig.json'
    }), metaEnv({
      FIREBASE_API_KEY: process.env.FIREBASE_API_KEY,
      FIREBASE_AUTH_DOMAIN: process.env.FIREBASE_AUTH_DOMAIN,
      FIREBASE_PROJECT_ID: process.env.FIREBASE_PROJECT_ID,
      FIREBASE_STORAGE_BUCKET: process.env.FIREBASE_STORAGE_BUCKET,
      FIREBASE_MESSAGING_SENDER_ID: process.env.FIREBASE_MESSAGING_SENDER_ID,
      FIREBASE_MESSAGING_APP_ID: process.env.FIREBASE_MESSAGING_APP_ID})],
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
