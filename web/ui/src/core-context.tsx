import { App } from '@matsunoki/core'
import { createContext, useContext } from 'solid-js'

export const CoreContext = createContext<App | "NotProvided">("NotProvided")

export const useCore = (): App => {
  const ctx = useContext(CoreContext)
  if(ctx === 'NotProvided') throw new Error("Core context is not provided")
  return ctx
}
