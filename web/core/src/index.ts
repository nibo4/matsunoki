import { BehaviorSubject, Subject } from 'rxjs'
import { signUp, verify } from '@matsunoki/api-client'
import {SignedInObserver, signIn} from './user-action';
import { firebaseSignInWithProvider } from './infra/firebase-auth-provider'
import { handleSignInCallback, HandleSignInCallbackObserver } from './system-action';



export * from "./session";
export * as userAction from "./user-action";
export * as systemAction from "./system-action";

export type App = {
  readModels:  {
    handleSignInCallbackObserver: HandleSignInCallbackObserver
  },
  systemActions: {
    handleSignInCallback: ReturnType<typeof handleSignInCallback>
  },
  userActions: {
    signIn: ReturnType<typeof signIn>
  }
}

export const initForProduction: () => App = () => {
  return {
    readModels: {
      handleSignInCallbackObserver: new Subject()
    },
    systemActions: {
      handleSignInCallback: handleSignInCallback({
        signUp: signUp({

        } as any),
        verify: verify({

        } as any),
        signInObserver: new Subject()
      })
    },
    userActions: {
      signIn: signIn({
        signInProvider: firebaseSignInWithProvider,
        signedInObserver: new Subject()
      })
    }
  }
}
