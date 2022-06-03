import { Err, Ok, Result } from "ts-results";
import { z } from "zod";
import {
  Config,
  buildURL,
  buildHeader,
  UnknownError,
  buildUnknownError,
} from "./internal";

export type SignUpError = {
  kind: string;
  key: string;
};

export type SignUpResponse = {
  userId: string;
  name: string;
};

export type SignUp = (
  config: Config
) => Promise<Result<SignUpResponse, SignUpError | UnknownError>>;

export const responseHandler = (
  a: any
): Result<SignUpResponse, SignUpError | UnknownError> => {
  try {
    const schema = z.object({
      user_id: z.string(),
      name: z.string(),
    });
    const parsed = schema.parse(a);
    return Ok({
      userId: parsed.user_id,
      name: parsed.name,
    });
  } catch (e) {
    return Err(buildUnknownError(e));
  }
};

export const signUp: SignUp = async (config) => {
  const response = await config.fetch(buildURL("sign_up", config), {
    method: "POST",
    mode: "cors",
    headers: buildHeader(config),
  });

  return responseHandler(response);
};
