import type { Config } from "@matsunoki/api-client";

export type SignInSession =
  | {
      kind: "beforeSignIn";
    }
  | {
      kind: "signed";
      apiKey: string;
    };

export class BeforeInitializeErrorError extends Error {
  constructor(...params: any[]) {
    super(...params);
    this.name = "BeforeInitializeErrorError";
  }
}

export const buildApiClientConfig = (session: SignInSession): Config => {
  if (session.kind !== "signed") throw new BeforeInitializeErrorError();

  return {
    fetch: window.fetch,
    authorizationToken: session.apiKey,
    host: import.meta.env.ACCOUNT_API_HOST,
  };
};
