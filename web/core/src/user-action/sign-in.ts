import { Observer } from "rxjs";
import { Err, Ok, Result } from "ts-results";
import { buildCoreUnknownError, CoreUnkownError } from "../error";
import { SignInWithProvider } from "../interface";

export type SignInResult = Result<string, CoreUnkownError>;
type SignedInObserver = Observer<SignInResult>;

type SignInDependencies = {
  signInProvider: SignInWithProvider;
  signedInObserver: SignedInObserver;
};

type SignIn = () => Promise<void>;

export const signIn: (deps: SignInDependencies) => SignIn =
  (deps: SignInDependencies) => async (): Promise<void> => {
    await deps
      .signInProvider("google")
      .then((idToken) => {
        deps.signedInObserver.next(Ok(idToken));
        deps.signedInObserver.complete();
      })
      .catch((err) => {
        deps.signedInObserver.next(Err(buildCoreUnknownError(err)));
      });
  };
