import { Err, Ok, Result } from "ts-results";
import { z } from "zod";
import { Config, UnknownError } from "./shared";
import { buildURL, buildHeader, buildUnknownError } from "./internal";

export type SignUpError = {
  kind: string;
  key: string;
};

export type SignUpResponse = {
  userId: string;
  name: string;
};

export type SignUp = () => Promise<
  Result<SignUpResponse, SignUpError | UnknownError>
>;

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

type Dependencies = {
  config: Config;
};

export const signUp =
  (deps: Dependencies): SignUp =>
  async () => {
    const response = await deps.config.fetch(buildURL("sign_up", deps.config), {
      method: "POST",
      mode: "cors",
      headers: buildHeader(deps.config),
    });

    return responseHandler(response);
  };
