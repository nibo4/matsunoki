import { responseHandler } from "./sign-up";

describe("responseHandler", () => {
  it("error", () => {
    expect(responseHandler({ dummy: 123 }).err).toStrictEqual(true);
  });

  it("ok", () => {
    const actual = responseHandler({ user_id: "xxx", name: "yyyy" });
    expect(actual.ok).toStrictEqual(true);
    expect(actual.val).toStrictEqual({
      userId: "xxx",
      name: "yyyy",
    });
  });
});
