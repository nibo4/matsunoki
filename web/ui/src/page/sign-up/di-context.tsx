import { Component, createContext, JSXElement, useContext } from "solid-js";

type Dependencies = {};
const SignUpDIContext = createContext<Dependencies>();

export const useSignUpDIContext = (): Dependencies => {
  const ctx = useContext(SignUpDIContext);
  if (ctx == null) {
    throw new Error("SignUpDIContext is not provided");
  }
  return ctx;
};

export const SignUpDIContextProviderForPreview: Component<{
  children: JSXElement;
}> = (props) => {
  return (
    <SignUpDIContext.Provider value={{}}>
      {props.children}
    </SignUpDIContext.Provider>
  );
};
