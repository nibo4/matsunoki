import { Subject } from "rxjs";
import type {
  SignUpResponse,
  SignUpError,
  UnknownError,
  SignUp,
} from "@matsunoki/api-client";
import { Ok, Result } from "ts-results";

export const signUpUserActionSubject = new Subject<
  Result<SignUpResponse, SignUpError | UnknownError>
>();

type SignUpUserActionDependencies = {
  signUp: SignUp;
};

export type SignUpUserAction = () => Promise<void>;

export const signUpUserAction =
  (deps: SignUpUserActionDependencies) => async (): Promise<void> => {
    const response = await deps.signUp();

    signUpUserActionSubject.next(response);
  };

export const signUpUserActionForPreview = (): SignUpUserAction => {
  return signUpUserAction({
    signUp: () => {
      console.info("dispatched signup uesr-action");
      return Promise.resolve(Ok({ userId: "xxx", name: "yyyy" }));
    },
  });
};
