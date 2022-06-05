import { Observer } from 'rxjs';
import { SignInWithProvider } from '../interface'

type SignedInObserver = Observer<string>

type SignInDependencies = {
  signInProvider: SignInWithProvider,
  signedInObserver: SignedInObserver
};

type SignIn = () => Promise<void>;

export const signIn: (deps: SignInDependencies) => SignIn =
  (deps: SignInDependencies) => async (): Promise<void> => {
    deps.signInProvider("google")
      .then(idToken => {
        deps.signedInObserver.next(idToken)
        deps.signedInObserver.complete()
      })
      .catch(err => {
        deps.signedInObserver.error(err)
      })
  };
