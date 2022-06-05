import {Subject} from "rxjs";
import {
  signIn
} from "./sign-in";

describe("#handleSignUpCallback", () => {
  describe("when success", () => {
    it("success result flow in the subject", () => {
      const subject = new Subject()

      subject.subscribe((token) => {
        expect(token).toStrictEqual("fooo")
      })

      signIn({
        signInProvider: () => Promise.resolve("fooo"),
        signedInObserver: subject
      })
    });
  });

  describe("when failed", () => {
    it("success result flow in the subject", () => {
      const subject = new Subject()

      subject.subscribe({
        error: (err) => {
          expect(err).toStrictEqual("fooo")
        }
      })

      signIn({
        signInProvider: () => Promise.reject("fooo"),
        signedInObserver: subject
      })
    });
  });
});
