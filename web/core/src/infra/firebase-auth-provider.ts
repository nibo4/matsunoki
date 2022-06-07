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
  const credential = GoogleAuthProvider.credentialFromResult(signInResult);
  if (credential == null || credential.idToken == null) {
    throw new Error("token is required(actual: null)");
  }
  return credential.idToken;
};
