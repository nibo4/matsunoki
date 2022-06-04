import { Subject } from "rxjs";
import type {
  SignUpResponse,
  SignUpError,
  UnknownError,
  SignUp,
} from "@matsunoki/api-client";
import { Ok, Result } from "ts-results";

export const handleSignUpCallbackSystemActionSubject = new Subject<
  Result<SignUpResponse, SignUpError | UnknownError>
>();

type HandleSignUpSystemActionDependencies = {
  signUp: SignUp;
};

export type HandleSignUpCallbackSystemAction = () => Promise<void>;

export const handleSignUpCallbackSystemAction =
  (deps: HandleSignUpSystemActionDependencies) => async (): Promise<void> => {
    const response = await deps.signUp();

    handleSignUpCallbackSystemActionSubject.next(response);
  };

export const handleSignUpCallbackSystemActionForPreview =
  (): HandleSignUpCallbackSystemAction => {
    return handleSignUpCallbackSystemAction({
      signUp: () => {
        console.info("dispatched signup uesr-action");
        return Promise.resolve(Ok({ userId: "xxx", name: "yyyy" }));
      },
    });
  };
