import type { Config } from "@matsunoki/api-client";
import { BehaviorSubject } from "rxjs";

export type SignInSession =
  | {
      kind: "beforeSignIn";
    }
  | {
      kind: "signed";
      apiKey: string;
    };

export type SessionStore = BehaviorSubject<SignInSession>;

export class BeforeInitializeErrorError extends Error {
  constructor(...params: any[]) {
    super(...params);
    this.name = "BeforeInitializeErrorError";
  }
}

export const buildGetConfig = (sessionStore: SessionStore): (() => Config) => {
  return () => {
    const session = sessionStore.getValue();

    if (session.kind === "beforeSignIn") throw new BeforeInitializeErrorError();

    return {
      fetch: window.fetch.bind(window),
      authorizationToken: session.apiKey,
      host: import.meta.env.ACCOUNT_API_HOST,
    };
  };
};
