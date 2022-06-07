interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string;
  readonly ACCOUNT_API_HOST: string;

  readonly FIREBASE_API_KEY: string;
  readonly FIREBASE_AUTH_DOMAIN: string;
  readonly FIREBASE_PROJECT_ID: string;
  readonly FIREBASE_STORAGE_BUCKET: string;
  readonly FIREBASE_MESSAGING_SENDER_ID: string;
  readonly FIREBASE_MESSAGING_APP_ID: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
