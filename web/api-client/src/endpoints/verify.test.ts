import { responseHandler } from "./verify";

describe("responseHandler", () => {
  it("error", () => {
    expect(responseHandler({ dummy: 123 }).err).toStrictEqual(true);
  });

  it("ok", () => {
    const actual = responseHandler({ id: 'foo' });
    expect(actual.ok).toStrictEqual(true);
    expect(actual.val).toStrictEqual({
      userId: 'foo'
    });
  });
});
