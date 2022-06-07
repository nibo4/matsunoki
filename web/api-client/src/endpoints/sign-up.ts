import { Err, Ok, Result } from "ts-results";
import { z } from "zod";
import { Config, UnknownError } from "./shared";
import { buildURL, buildHeader, buildUnknownError } from "./internal";

export type SignUpError = {
  kind: "AlreadyExist";
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

export const responseErrorHandler = (
  a: any
): Result<never, SignUpError | UnknownError> => {
  try {
    const schema = z.object({
      kind: z.string(),
      key: z.string(),
    });
    const parsed = schema.parse(a);
    if (parsed.kind === "already_exist") {
      return Err({ kind: "AlreadyExist" });
    }
    return Err(buildUnknownError(a));
  } catch (e) {
    return Err(buildUnknownError(e));
  }
};

type Dependencies = {
  getConfig: () => Config;
};

export const signUp =
  (deps: Dependencies): SignUp =>
  async () => {
    const config = deps.getConfig();
    const response = await config.fetch(buildURL("sign_up", config), {
      method: "POST",
      mode: "cors",
      headers: buildHeader(config),
    });

    if (response.ok) {
      return responseHandler(await response.json());
    }
    return responseErrorHandler(await response.json());
  };
