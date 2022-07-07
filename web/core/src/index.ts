import { BehaviorSubject, Subject } from "rxjs";
import { signUp, verify } from "@matsunoki/api-client";
import { SignedInObservable, signIn, SignInResult } from "./user-action";
import { firebaseSignInWithProvider } from "./infra/firebase-auth-provider";
import { SignInSession, buildGetConfig } from "./session";

export * from "./session";
export * as userAction from "./user-action";
export * as systemAction from "./system-action";

export type App = {
  readModels: {
    signedInObservable: SignedInObservable;
  };
  systemActions: {};
  userActions: {
    signIn: ReturnType<typeof signIn>;
  };
};

export const initForProduction: () => App = () => {
  const signedInSubject = new Subject<SignInResult>();
  const sessionStore = new BehaviorSubject<SignInSession>({
    kind: "beforeSignIn",
  });
  return {
    readModels: {
      signedInObservable: signedInSubject,
    },
    systemActions: {},
    userActions: {
      signIn: signIn({
        sessionStore,
        signedInObserver: signedInSubject,
        signInProvider: firebaseSignInWithProvider,
        signUp: signUp({
          getConfig: buildGetConfig(sessionStore),
        }),
        verify: verify({
          getConfig: buildGetConfig(sessionStore),
        }),
      }),
    },
  };
};
