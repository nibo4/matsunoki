export type CoreUnkownError = {
  kind: "core:unknown-error";
  e: any;
};

export const buildCoreUnknownError = (e: any): CoreUnkownError => {
  return {
    kind: "core:unknown-error",
    e: e,
  };
};
