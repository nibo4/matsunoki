export type Fetch = typeof window.fetch;

export type Config = {
  fetch: Fetch;
  host: string;
  authorizationToken: string;
};

export type UnknownError = {
  kind: "api-client:unknown-error";
  e: any;
};
