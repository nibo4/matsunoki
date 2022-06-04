import { Err, Ok } from "ts-results";
import {
  handleSignUpCallback,
  handleSignUpCallbackSubject,
} from "./handle-sign-up-callback";

describe("#handleSignUpCallback", () => {
  describe("when success", () => {
    it("success result flow in the handleSignUpCallbackSubject", () => {
      const dummyData = {
        userId: "xxx",
        name: "yyyy",
      };
      handleSignUpCallbackSubject.subscribe((result) => {
        expect(result.ok).toStrictEqual(true);
        expect(result.val).toStrictEqual(dummyData);
      });

      handleSignUpCallback({
        signUp: () => Promise.resolve(Ok(dummyData)),
      });
    });
  });

  describe("when failed", () => {
    it("success result flow in the handleSignUpCallbackSubject", () => {
      handleSignUpCallbackSubject.subscribe((result) => {
        expect(result.err).toStrictEqual(true);
        expect(result.val).toStrictEqual({
          kind: "api-client:unknown-error",
          e: 12,
        });
      });

      handleSignUpCallback({
        signUp: () =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: 12 })),
      });
    });
  });
});
