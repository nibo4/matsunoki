export type Provider = "google";
export type AccessToken = string;

export type SignInWithProvider = (p: Provider) => Promise<AccessToken>;
