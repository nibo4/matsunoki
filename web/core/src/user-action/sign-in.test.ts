import { SignUp } from "@matsunoki/api-client";
import { BehaviorSubject } from "rxjs";
import { Err, Ok } from "ts-results";
import { SignInSession } from "../session";
import { signIn } from "./sign-in";

describe("#signIn", () => {
  describe("success pattern", () => {
    it("should shed success result in the subject", async () => {
      const dummyData = {
        userId: "foo",
        name: "yyyy",
      };
      const subject = new BehaviorSubject<any>(null);
      const sessionSubject = new BehaviorSubject<SignInSession>({
        kind: "beforeSignIn" as const,
      });

      await signIn({
        sessionStore: sessionSubject,
        signInProvider: () => Promise.resolve("fooo"),
        signUp: () => Promise.resolve(Ok(dummyData)),
        verify: () => Promise.resolve(Ok({ userId: "foo" })),
        signedInObserver: subject,
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
      const subject = new BehaviorSubject<any>(null);
      const sessionSubject = new BehaviorSubject<SignInSession>({
        kind: "beforeSignIn" as const,
      });

      await signIn({
        sessionStore: sessionSubject,
        signInProvider: () => Promise.resolve("fooo"),
        signUp: () => Promise.resolve(Ok(dummyData)),
        verify: () => Promise.resolve(Err({ kind: "UserNotFound" })),
        signedInObserver: subject,
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
    it("should shed error in the signedInObserver when failed connect psign in provider", async () => {
      const dummyData = {
        userId: "foo",
        name: "yyyy",
      };

      const subject = new BehaviorSubject<any>(null);
      const sessionSubject = new BehaviorSubject<SignInSession>({
        kind: "beforeSignIn" as const,
      });

      await signIn({
        sessionStore: sessionSubject,
        signInProvider: () => Promise.reject("fooo"),
        signUp: () => Promise.resolve(Ok(dummyData)),
        verify: () => Promise.resolve(Ok({ userId: "foo" })),
        signedInObserver: subject,
      })();

      expect(subject.getValue()?.err).toStrictEqual(true);
    });

    it("should shed error result in the signedInObserver when verify === user-not-found and failed signUp", async () => {
      const subject = new BehaviorSubject<any>(null);
      const sessionSubject = new BehaviorSubject<SignInSession>({
        kind: "beforeSignIn" as const,
      });

      await signIn({
        sessionStore: sessionSubject,
        signInProvider: () => Promise.resolve("fooo"),
        verify: () => Promise.resolve(Err({ kind: "UserNotFound" })),
        signUp: (): ReturnType<SignUp> =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: "foo" })),
        signedInObserver: subject,
      })();

      expect(subject.getValue()?.err).toStrictEqual(true);
    });

    it("should shed error result in the signedInObserver when failed verify and signUp", async () => {
      const subject = new BehaviorSubject<any>(null);
      const sessionSubject = new BehaviorSubject<SignInSession>({
        kind: "beforeSignIn" as const,
      });

      await signIn({
        sessionStore: sessionSubject,
        signInProvider: () => Promise.resolve("fooo"),
        verify: () =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: 12 })),
        signUp: (): ReturnType<SignUp> =>
          Promise.resolve(Err({ kind: "api-client:unknown-error", e: "foo" })),
        signedInObserver: subject,
      })();

      expect(subject.getValue()?.err).toStrictEqual(true);
    });
  });
});
