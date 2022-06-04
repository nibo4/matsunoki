import { Subject } from "rxjs";
import type {
  SignUpResponse,
  SignUpError,
  UnknownError,
  SignUp,
} from "@matsunoki/api-client";
import { Ok, Result } from "ts-results";

export const handleSignUpCallbackSubject = new Subject<
  Result<SignUpResponse, SignUpError | UnknownError>
>();

type HandleSignUpDependencies = {
  signUp: SignUp;
};

export type HandleSignUpCallback = () => Promise<void>;

export const handleSignUpCallback =
  (deps: HandleSignUpDependencies) => async (): Promise<void> => {
    const response = await deps.signUp();

    handleSignUpCallbackSubject.next(response);
  };

export const handleSignUpCallbackForPreview = (): HandleSignUpCallback => {
  return handleSignUpCallback({
    signUp: () => {
      console.info("dispatched signup uesr-action");
      return Promise.resolve(Ok({ userId: "xxx", name: "yyyy" }));
    },
  });
};
