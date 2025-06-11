# WebAssembly対応Kubernetesクラスター用サンプルアプリ

このプロジェクトは、WebAssembly（WASM）を使用したHTTPサーバーとクライアントのサンプル実装です。
ハッカソンでWASMベースのKubernetesクラスターを構築するためのデモンストレーション用として作成されています。

## プロジェクト構成

```text
hackathon2506/
├── README.md           # このファイル
├── .gitignore
├── server/            # WASMサーバー (Rust)
│   ├── README.md      # サーバー詳細ドキュメント
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── Dockerfile
└── client/            # Webクライアント (React SPA)
    ├── README.md      # クライアント詳細ドキュメント
    ├── package.json
    ├── index.html
    └── src/
```

## 概要

このプロジェクトは軽量で高速なWASMベースのHTTPサーバーと、シンプルなクライアントを組み合わせた構成になっています。

### 主な特徴

- **軽量・高速**: WebAssemblyによる高速起動と低リソース消費
- **セキュア**: WASMサンドボックス環境での安全な実行
- **ポータブル**: WASI対応により様々な環境で動作
- **モダン**: React + Viteによる高速な開発体験

## クイックスタート

### サーバーの起動

```shell
cd server
# 詳細な実行手順は server/README.md を参照
```

### クライアントの起動

```shell
cd client
npm install
npm run dev
# 詳細な実行手順は client/README.md を参照
```

## ドキュメント

各コンポーネントの詳細な情報については、以下のドキュメントを参照してください：

- [サーバー詳細](./server/README.md) - WASMサーバーのビルド・実行・デプロイ手順
- [クライアント詳細](./client/README.md) - Reactクライアントの開発・ビルド手順

## ハッカソンでの利用

このサンプルは以下の用途で活用できます：

- **マイクロサービスアーキテクチャ**: 軽量なサービス
- **エッジコンピューティング**: 高速起動が必要な環境で
- **コンテナ代替**: より安全で軽量なワークロード
- **マルチプラットフォーム**: ノードアーキテクチャを問わずデプロイ可能

## 技術スタック

### サーバー

- Rust + WebAssembly (WASI)
- WasmEdgeランタイム

### クライアント

- React + Vite
- Modern JavaScript/CSS

## ライセンス

このプロジェクトはハッカソン用のサンプルコードです。
