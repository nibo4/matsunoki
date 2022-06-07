import { Observer, from, pipe } from "rxjs";
import type {
  SignUpError,
  SignUp,
  Verify,
  VerifyError,
} from "@matsunoki/api-client";
import { Err, Ok, Result } from "ts-results";
import {buildCoreUnknownError, CoreUnkownError} from "../error";

export type SignInUser = {kind: "NewUser", userId: string, name: string} | {kind: "ExistingUser", userId: string}

export type SignInResult =
  Result<SignInUser, SignUpError | VerifyError | CoreUnkownError>

type SignInObserver = Observer<SignInResult>;

type HandleSignInDependencies = {
  signUp: SignUp;
  verify: Verify
  signInObserver: SignInObserver
};

export type HandleSignInCallback = () => Promise<void>;

export const handleSignInCallback =
  (deps: HandleSignInDependencies): HandleSignInCallback => async () => {
    const verifyResult = await deps.verify()

    if(verifyResult.err && verifyResult.val.kind === 'UserNotFound') {
      (await deps.signUp())
        .map(val => {
          deps.signInObserver.next(Ok({kind: "NewUser", name: val.name, userId: val.userId}))
        })
        .mapErr((err) => {
          deps.signInObserver.next(Err(buildCoreUnknownError(err)))
        });
      return
    }

    if(verifyResult.err) {
      deps.signInObserver.next(verifyResult.mapErr(err => buildCoreUnknownError(err)))
      return
    }

    deps.signInObserver.next(Ok({kind: 'ExistingUser', userId: verifyResult.val.userId}));
  };
