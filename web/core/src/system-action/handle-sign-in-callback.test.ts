import { Subject } from "rxjs";
import { Err, Ok } from "ts-results";
import {
  handleSignInCallback,  SignInResult,
} from "./handle-sign-in-callback";

describe("#handleSignInCallback", () => {
  describe("when success", () => {
    it("success result flow in the handleSignInCallbackSubject", () => {
      const dummyData = {
        userId: "xxx",
        name: "yyyy",
      };
      const subject = new Subject<SignInResult>();

      subject.subscribe((result) => {
        expect(result.ok).toStrictEqual(true);
        expect(result.val).toStrictEqual(dummyData);
      });

      handleSignInCallback({
        signUp: () => Promise.resolve(Ok(dummyData)),
        signInObserver: subject
      });
    });
  });

  describe("when failed", () => {
    it("success result flow in the handleSignInCallbackSubject", () => {
      const subject = new Subject<SignInResult>();

      subject.subscribe((result) => {
        expect(result.err).toStrictEqual(true);
        expect(result.val).toStrictEqual({
          kind: "api-client:unknown-error",
          e: 12,
        });
      });

      handleSignInCallback({
        signUp: () =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: 12 })),
        signInObserver: subject
      });
    });
  });
});
