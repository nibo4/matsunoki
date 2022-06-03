import { Component } from "solid-js";
import { SignUpDIContextProviderForPreview } from "./di-context";
import { SignUpPage } from ".";

export const Story: Component = () => {
  return (
    <SignUpDIContextProviderForPreview>
      <SignUpPage />
    </SignUpDIContextProviderForPreview>
  );
};

export default Story;
