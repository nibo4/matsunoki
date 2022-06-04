import { Ok, Result } from "ts-results";

type SignInDependencies = {};

type SignIn = () => Promise<void>

export const signIn: (deps: SignInDependencies) => SignIn =
  (deps: SignInDependencies) => async (): Promise<void> => {
    throw new Error("unimplemented!")
  };

export const signInForPreview = (): SignIn => {
  return signIn({});
};
