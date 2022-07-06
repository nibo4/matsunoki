import { Component, createContext, JSXElement, useContext } from "solid-js";

type Dependencies = {
  navigateToRoot: () => void;
};
export const SignUpDIContext = createContext<Dependencies>();

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
    <SignUpDIContext.Provider
      value={{
        navigateToRoot: () => {
          console.log("to root");
        },
      }}
    >
      {props.children}
    </SignUpDIContext.Provider>
  );
};
