import { BehaviorSubject } from "rxjs";
import { Err, Ok } from "ts-results";
import { SignUp } from "@matsunoki/api-client";
import { handleSignInCallback, SignInResult } from "./handle-sign-in-callback";

describe("#handleSignInCallback", () => {
  describe("success pattern", () => {
    it("should shed success result in the handleSignInCallbackSubject", async () => {
      const dummyData = {
        userId: "foo",
        name: "yyyy",
      };
      const subject = new BehaviorSubject<SignInResult | null>(null);

      await handleSignInCallback({
        signUp: () => Promise.resolve(Ok(dummyData)),
        verify: () => Promise.resolve(Ok({ userId: "foo" })),
        signInObserver: subject,
      })();

      expect(subject.getValue()?.ok).toStrictEqual(true);
      expect(subject.getValue()?.val).toStrictEqual({
        kind: "ExistingUser",
        userId: "foo",
      });
    });

    it("should shed success result in the signInObserver when new user", async () => {
      const dummyData = {
        userId: "xxx",
        name: "yyyy",
      };
      const subject = new BehaviorSubject<SignInResult | null>(null);

      await handleSignInCallback({
        signUp: () => Promise.resolve(Ok(dummyData)),
        verify: () => Promise.resolve(Err({ kind: "UserNotFound" })),
        signInObserver: subject,
      })();

      expect(subject.getValue()?.ok).toStrictEqual(true);
      expect(subject.getValue()?.val).toStrictEqual({
        kind: "NewUser",
        name: "yyyy",
        userId: "xxx",
      });
    });
  });

  describe("failed pattern", () => {
    it("should shed error result in the handleSignInCallbackSubject when failed verify and signUp", async () => {
      const subject = new BehaviorSubject<SignInResult | null>(null);

      await handleSignInCallback({
        verify: () => Promise.resolve(Err({ kind: "UserNotFound" })),
        signUp: (): ReturnType<SignUp> =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: "foo" })),
        signInObserver: subject,
      })();

      expect(subject.getValue()?.err).toStrictEqual(true);
      expect(subject.getValue()?.val.kind).toStrictEqual("core:unknown-error");
    });

    it("should shed error result in the handleSignInCallbackSubject when failed verify is unkonwon error", async () => {
      const subject = new BehaviorSubject<SignInResult | null>(null);

      await handleSignInCallback({
        verify: () =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: 12 })),
        signUp: (): ReturnType<SignUp> =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: "foo" })),
        signInObserver: subject,
      })();

      expect(subject.getValue()?.err).toStrictEqual(true);
      expect(subject.getValue()?.val.kind).toStrictEqual("core:unknown-error");
    });
  });
});
