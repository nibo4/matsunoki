import {Err, Ok} from 'ts-results'
import { signUpUserAction, signUpUserActionSubject } from './sign-up'

describe("#signUpUserAction", () => {
  describe("when success", () => {
    it("success result flow in the signUpUserActionSubject", () => {
      const dummyData = {
        userId: 'xxx',
        name: 'yyyy'
      }
      signUpUserActionSubject.subscribe((result) => {
        expect(result.ok).toStrictEqual(true)
        expect(result.val).toStrictEqual(dummyData)
      })

      signUpUserAction({
        signUp: () => Promise.resolve(Ok(dummyData))
      })
    })
  })

  describe("when failed", () => {
    it("success result flow in the signUpUserActionSubject", () => {
      signUpUserActionSubject.subscribe((result) => {
        expect(result.err).toStrictEqual(true)
        expect(result.val).toStrictEqual({ kind: 'api-client:unknown-error', e: 12 })
      })

      signUpUserAction({
        signUp: () => Promise.resolve(Err({ kind: 'api-client:unknown-error', e: 12 }))
      })
    })
  })
})

