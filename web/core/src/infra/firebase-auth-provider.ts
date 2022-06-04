import type { SignInWithProvider } from "../interface/";

export const firebaseSignInWithProvider: SignInWithProvider = (_provider) => {
  return Promise.resolve("foo");
};
