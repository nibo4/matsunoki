import {Component, createContext, JSXElement, useContext} from "solid-js";
import { SignUpUserAction, signUpUserActionForPreview } from '@matsunoki/core'

type Dependencies = {
  signUp: SignUpUserAction
}
const SignUpDIContext = createContext<Dependencies>()

export const useSignUpDIContext = (): Dependencies => {
  const ctx = useContext(SignUpDIContext)
  if(ctx == null) {
    throw new Error("SignUpDIContext is not provided")
  }
  return ctx
}

export const SignUpDIContextProviderForPreview: Component<{children: JSXElement}> = (props) => {
  return (
    <SignUpDIContext.Provider value={{
      signUp: signUpUserActionForPreview()
    }}>
      {props.children}
    </SignUpDIContext.Provider>
  )
}
