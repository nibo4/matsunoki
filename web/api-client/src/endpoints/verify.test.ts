import { responseErrorHandler, responseHandler } from "./verify";

describe("responseHandler", () => {
  it("error", () => {
    expect(responseHandler({ dummy: 123 }).err).toStrictEqual(true);
  });

  it("ok", () => {
    const actual = responseHandler({ id: "foo" });
    expect(actual.ok).toStrictEqual(true);
    expect(actual.val).toStrictEqual({
      userId: "foo",
    });
  });
});

describe("responseErrorHandler", () => {
  it("error", () => {
    expect(
      responseErrorHandler({ kind: "user_not_found", key: "" }).val
    ).toStrictEqual({ kind: "UserNotFound" });
    expect(
      responseErrorHandler({ kind: "verify_failed", key: "" }).val
    ).toStrictEqual({ kind: "VerifyError" });
  });
});
