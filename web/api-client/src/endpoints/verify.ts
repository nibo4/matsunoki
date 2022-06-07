import { Err, Ok, Result } from "ts-results";
import { z } from "zod";
import { Config, UnknownError } from "./shared";
import { buildURL, buildHeader, buildUnknownError } from "./internal";

export type VerifyError =
  | {
      kind: "VerifyError";
    }
  | {
      kind: "UserNotFound";
    };

export type VerifyResponse = {
  userId: string;
};

export type Verify = () => Promise<
  Result<VerifyResponse, VerifyError | UnknownError>
>;

export const responseHandler = (
  a: any
): Result<VerifyResponse, VerifyError | UnknownError> => {
  try {
    const schema = z.object({
      id: z.string(),
    });
    const parsed = schema.parse(a);
    return Ok({
      userId: parsed.id,
    });
  } catch (e) {
    return Err(buildUnknownError(e));
  }
};

export const responseErrorHandler = (
  a: any
): Result<never, VerifyError | UnknownError> => {
  try {
    const schema = z.object({
      kind: z.string(),
      key: z.string(),
    });
    const parsed = schema.parse(a);
    switch (parsed.kind) {
      case "verify_failed":
        return Err({ kind: "VerifyError" });
      case "user_not_found":
        return Err({ kind: "UserNotFound" });
      default:
        return Err(buildUnknownError(parsed));
    }
  } catch (e) {
    return Err(buildUnknownError(e));
  }
};

type Dependencies = {
  config: Config;
};

export const verify =
  (deps: Dependencies): Verify =>
  async () => {
    const response = await deps.config.fetch(buildURL("verify", deps.config), {
      method: "POST",
      mode: "cors",
      headers: buildHeader(deps.config),
    });

    if (response.ok) {
      return responseHandler(await response.json());
    }

    return responseErrorHandler(await response.json());
  };
