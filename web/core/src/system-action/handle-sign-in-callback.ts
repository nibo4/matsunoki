import { Observer } from "rxjs";
import type {
  SignUpError,
  UnknownError,
  SignUp,
} from "@matsunoki/api-client";
import { Ok, Result } from "ts-results";

export type SignInResult = 
  Result<{}, SignUpError | UnknownError>

type SignInObserver = Observer<SignInResult>;

type HandleSignInDependencies = {
  signUp: SignUp;
  signInObserver: SignInObserver
};

export type HandleSignInCallback = () => Promise<void>;

export const handleSignInCallback =
  (deps: HandleSignInDependencies) => async (): Promise<void> => {
    const _response = await deps.signUp();

    deps.signInObserver.next(Ok({}));
  };
