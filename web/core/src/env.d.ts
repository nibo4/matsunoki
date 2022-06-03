interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string,
  readonly ACCOUNT_API_HOST: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
