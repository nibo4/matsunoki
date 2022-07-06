import { SignUp, Verify } from "@matsunoki/api-client";
import { Observable, Observer } from "rxjs";
import { Err, Ok, Result } from "ts-results";
import { buildCoreUnknownError, CoreUnkownError } from "../error";
import { SignInWithProvider } from "../interface";
import { SessionStore } from "../session";

export type SignedInUser =
  | { kind: "NewUser"; userId: string; name: string }
  | { kind: "ExistingUser"; userId: string };

export type SignInResult = Result<SignedInUser, CoreUnkownError>;
export type SignedInObserver = Observer<SignInResult>;
export type SignedInObservable = Observable<SignInResult>;

type SignInDependencies = {
  signUp: SignUp;
  verify: Verify;
  signInProvider: SignInWithProvider;
  signedInObserver: SignedInObserver;
  sessionStore: SessionStore;
};

type SignIn = () => Promise<void>;

export const signIn: (deps: SignInDependencies) => SignIn =
  (deps: SignInDependencies) => async (): Promise<void> => {
    try {
      const token = await deps.signInProvider("google");
      deps.sessionStore.next({ kind: "signed", apiKey: token });
      const verifyResult = await deps.verify();

      if (verifyResult.err && verifyResult.val.kind === "UserNotFound") {
        (await deps.signUp())
          .map((val) => {
            deps.signedInObserver.next(
              Ok({ kind: "NewUser", name: val.name, userId: val.userId })
            );
          })
          .mapErr((err) => {
            deps.signedInObserver.next(Err(buildCoreUnknownError(err)));
          });
        return;
      }

      if (verifyResult.err) {
        deps.signedInObserver.next(
          verifyResult.mapErr((err) => buildCoreUnknownError(err))
        );
        return;
      }

      deps.signedInObserver.next(
        Ok({ kind: "ExistingUser", userId: verifyResult.val.userId })
      );
    } catch (e) {
      deps.signedInObserver.next(Err(buildCoreUnknownError(e)));
    }
  };
