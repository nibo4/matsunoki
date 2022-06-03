import type { Config } from '@matsunoki/api-client'

export type Session = {
  kind: 'beforeInitialize'
} | {
  kind: 'initialized',
  apiKey: string
}


export class BeforeInitializeErrorError extends Error {
  constructor(...params: any[]) {
    super(...params)
    this.name = 'BeforeInitializeErrorError'
  }
}

export const buildApiClientConfig = (session: Session): Config => {
  if(session.kind !== 'initialized') throw new BeforeInitializeErrorError()

  return {
    fetch: window.fetch,
    authorizationToken: session.apiKey,
    host: import.meta.env.ACCOUNT_API_HOST
  }
}
