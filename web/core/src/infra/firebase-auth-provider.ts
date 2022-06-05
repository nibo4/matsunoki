import { initializeApp } from "firebase/app";
import { GoogleAuthProvider, getAuth, signInWithPopup } from "firebase/auth";
import type { SignInWithProvider } from "../interface/";
import { firebaseConfig } from "../firebase.config";

export const firebaseSignInWithProvider: SignInWithProvider = async (
  _provider
) => {
  initializeApp(firebaseConfig);
  const provider = new GoogleAuthProvider();
  const auth = getAuth();

  const signInResult = await signInWithPopup(auth, provider);
  const token = GoogleAuthProvider.credentialFromResult(signInResult);
  if (token == null || token.idToken == null) {
    throw new Error("token is required(actual: null)");
  }
  return token.idToken;
};
