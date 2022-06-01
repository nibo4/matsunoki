import { createI18nContext } from '@solid-primitives/i18n'

const dict = {
  ja:{
    signUp: 'サインアップ',
    signUpWithGoogle: 'Googleでサインアップ',
    signUpButtonLabel: 'サインアップ',
    description: 'Matsunokiのアカウントを作成します'
  }
}

export const SignUpI18nContext = createI18nContext(dict)
