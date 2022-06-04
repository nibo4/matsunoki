export type Provider = "google"
export type Token = string

export type SignInWithProvider = (p: Provider) => Promise<Token>
