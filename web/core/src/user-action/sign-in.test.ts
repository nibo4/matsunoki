import {BehaviorSubject } from "rxjs";
import {
  signIn, SignInResult
} from "./sign-in";

describe("#signIn", () => {
  describe("when success", () => {
    it("success result flow in the subject", async () => {
      const subject = new BehaviorSubject<SignInResult | null>(null)

      await signIn({
        signInProvider: () => Promise.resolve("fooo"),
        signedInObserver: subject
      })()

      expect(subject.getValue()?.ok).toStrictEqual(true)
      expect(subject.getValue()?.val).toStrictEqual("fooo")
    });
  });

  describe("when failed", () => {
    it("success result flow in the subject", async () => {
      const subject = new BehaviorSubject<SignInResult | null>(null)

      await signIn({
        signInProvider: () => Promise.reject("fooo"),
        signedInObserver: subject
      })()

      expect(subject.getValue()?.err).toStrictEqual(true)
    });
  });
});
