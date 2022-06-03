import { Component } from "solid-js";
import { SignUpPage } from ".";
import { SignUpDIContextProviderForPreview } from './di-context'

export const Story: Component = () => {
  return (
    <SignUpDIContextProviderForPreview>
      <SignUpPage />
    </SignUpDIContextProviderForPreview>
  )
};

export default Story;
