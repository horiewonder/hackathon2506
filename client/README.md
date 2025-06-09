# WASM HTTP Server Client

WebAssembly（WASM）で動作するHTTPサーバーをテストするためのReact SPAクライアントです。
ハッカソンでWASMベースのKubernetesクラスターをデモンストレーションする際に使用します。

## 必要な環境

- Node.js (18.0.0 以降)
- npm または yarn

## セットアップ

### 1. 依存関係のインストール

```bash
npm install
```

### 2. 開発サーバーの起動

```bash
npm run dev
```

デフォルトでは `http://localhost:5173` でクライアントが起動します。

## 使用方法

### 基本的な使い方

1. **WASMサーバーを起動**: まず、`../server` ディレクトリでWASMサーバーを起動してください
2. **ブラウザでアクセス**: `http://localhost:5173` をブラウザで開く
3. **サーバーURLを設定**: デフォルトは `http://localhost:1234` です
4. **メッセージを入力**: 送信したいテキストを入力
5. **POST送信**: ボタンを押してサーバーにリクエストを送信
6. **レスポンス確認**: サーバーからの応答がデコードされて表示されます

## 技術スタック

- **フレームワーク**: React 19.1.0
- **ビルドツール**: Vite 6.3.5
- **言語**: JavaScript (ES2022+)
- **スタイリング**: CSS3 (モダンなグラデーションとアニメーション)
- **HTTP通信**: Fetch API
- **開発支援**: ESLint, React Dev Tools対応

## 開発用コマンド

```bash
# 開発サーバー起動
npm run dev

# プロダクションビルド
npm run build

# ビルド結果のプレビュー
npm run preview

# コードリンティング
npm run lint
```

## ライセンス

このプロジェクトはハッカソン用のサンプルコードです。
