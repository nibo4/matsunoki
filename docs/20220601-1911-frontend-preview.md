## Decision

我々のプロジェクトはフロントエンドの動作確認として Storybook のようなツールを自作し、preview環境として使う

このpreview環境は APIサーバーや認証サービスとの接続はせず、そのサーバーのみで動くようにコードを書く

## Rationale

フロントエンドの開発をするにあたって次のような問題に遭遇します

- バックエンドの実装に依存しないフロントエンドのテスト環境がない
- バックエンドのAPIと密結合になっており、APIの実装が終わらないと画面の実装ができない

これらの問題を解決するために、APIなど実行依存がある部分をDependencyInjectionによって制御しつつ、APIの実装に依存しなくても動作確認できるアプリケーションとしてプレビュー環境を実装します

## Status

Accept

## Consequences

バックエンドと繋ぎ込まずに画面の開発ができるようになった