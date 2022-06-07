import { BehaviorSubject, Subject } from "rxjs";
import { signUp, verify } from "@matsunoki/api-client";
import { SignedInObserver, signIn } from "./user-action";
import { firebaseSignInWithProvider } from "./infra/firebase-auth-provider";
import { SignInSession, buildGetConfig } from "./session";

export * from "./session";
export * as userAction from "./user-action";
export * as systemAction from "./system-action";

export type App = {
  readModels: {
    signedInObserver: SignedInObserver;
  };
  systemActions: {};
  userActions: {
    signIn: ReturnType<typeof signIn>;
  };
};

export const initForProduction: () => App = () => {
  const signedInObserver = new Subject();
  const sessionStore = new BehaviorSubject<SignInSession>({
    kind: "beforeSignIn",
  });
  return {
    readModels: {
      signedInObserver,
    },
    systemActions: {},
    userActions: {
      signIn: signIn({
        sessionStore,
        signedInObserver,
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
