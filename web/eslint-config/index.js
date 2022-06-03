module.exports = {
  plugins: [
    "@typescript-eslint",
    "import",
    "solid",
  ],
  extends: [
    "eslint:recommended",
    "plugin:solid/recommended",
    "plugin:import/typescript",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    "sourceType": "module",
    "project": "./tsconfig.json"
  },
  env: {
    browser: true,
    node: true,
    es6: true,
  },
  rules: {
    '@typescript-eslint/no-unused-vars': ['error', { varsIgnorePattern: "^(_.+)|_$", argsIgnorePattern: "^(_.+)|_$" }],
    'no-undef': ['off'],
    '@typescript-eslint/naming-convention': [
      'error',
      {
        selector: 'default',
        format: ['camelCase', 'PascalCase', 'UPPER_CASE'],
      },
      {
        selector: 'property',
        format: null,
        filter: '^(_|__html)$',
      },
      {
        selector: 'parameter',
        format: null,
        filter: '^_',
      },
      {
        selector: 'variable',
        format: null,
        filter: '^_',
      },
      {
        selector: 'variableLike',
        format: ['camelCase', 'PascalCase', 'UPPER_CASE'],
      },
      {
        selector: 'typeLike',
        format: ['PascalCase', 'UPPER_CASE'],
      },
      {
        selector: 'parameter',
        format: ['camelCase', 'PascalCase'],
      },
      {
        selector: 'memberLike',
        modifiers: ['private'],
        format: ['camelCase'],
      },
    ],
    '@typescript-eslint/no-use-before-define': 'error',
    "@typescript-eslint/explicit-function-return-type": ['error', {
      "allowExpressions": true,
      "allowTypedFunctionExpressions": true
    }],
    'no-unused-vars': 'off',
    'import/order': 'error',
    'eqeqeq': ['error', 'smart'],
  }
}
