export type Fetch = typeof window.fetch

export type Config = {
  fetch: Fetch,
  host: string,
  authorizationToken: string
}

export type UnknownError = {
  kind: 'api-client:unknown-error',
  e: any
}

export const buildUnknownError = (e: any): UnknownError => {
  return {
    e,
    kind: 'api-client:unknown-error',
  }
}

export const buildURL = (path: string, config: Config): string => {
  const url = new URL(config.host)
  url.pathname = path;
  return url.toString()
}

export const buildHeader = (config: Config): Record<string, string> => {
  return {
    'Content-Type': 'application/json',
    Authorization: `Bearer: ${config.authorizationToken}`
  }
}

