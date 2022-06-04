import { Err, Ok } from "ts-results";
import {
  handleSignUpCallbackSystemAction,
  handleSignUpCallbackSystemActionSubject,
} from "./handle-sign-up-callback";

describe("#handleSignUpCallbackSystemAction", () => {
  describe("when success", () => {
    it("success result flow in the handleSignUpCallbackSystemActionSubject", () => {
      const dummyData = {
        userId: "xxx",
        name: "yyyy",
      };
      handleSignUpCallbackSystemActionSubject.subscribe((result) => {
        expect(result.ok).toStrictEqual(true);
        expect(result.val).toStrictEqual(dummyData);
      });

      handleSignUpCallbackSystemAction({
        signUp: () => Promise.resolve(Ok(dummyData)),
      });
    });
  });

  describe("when failed", () => {
    it("success result flow in the handleSignUpCallbackSystemActionSubject", () => {
      handleSignUpCallbackSystemActionSubject.subscribe((result) => {
        expect(result.err).toStrictEqual(true);
        expect(result.val).toStrictEqual({
          kind: "api-client:unknown-error",
          e: 12,
        });
      });

      handleSignUpCallbackSystemAction({
        signUp: () =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: 12 })),
      });
    });
  });
});
