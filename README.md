# WASM HTTP Server - ハッカソン用サンプル

このプロジェクトは、WebAssembly（WASM）を使用したHTTPサーバーのサンプル実装です。
ハッカソンでWASMベースのKubernetesクラスターを構築するためのデモンストレーション用として作成されています。

## プロジェクト構成

```text
wasm_http_server/
├── README.md           # このファイル
├── .gitignore
├── server/            # WASMサーバー (Rust)
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── Dockerfile
└── client/            # Webクライアント (React SPA)
```

## 概要

このHTTPサーバーは以下の特徴を持っています：

- **軽量**: WebAssemblyバイナリとして動作するため、非常に軽量です
- **高速起動**: 従来のコンテナと比較して起動時間が大幅に短縮されます
- **セキュリティ**: WASMのサンドボックス環境で実行されるため、セキュリティが向上します
- **ポータビリティ**: WASI（WebAssembly System Interface）を使用することで、様々な環境で動作可能です

## 必要な環境

- Rust（2018 edition以降）
- rustup（wasm32-wasip1ターゲットの追加用）
- wasmedgeランタイム（実行用）

## セットアップ

### 1. WASMターゲットの追加

```shell
rustup target add wasm32-wasip1
```

### 2. サーバーのビルド

```shell
cd server
cargo build --target wasm32-wasip1 --release
```

## 実行方法

### ローカルでの実行

```shell
cd server
wasmedge target/wasm32-wasip1/release/http_server.wasm
```

デフォルトでは`localhost:1234`でサーバーが起動します。

### 環境変数での設定

ポート番号は環境変数`PORT`で変更可能です：

```shell
cd server
PORT=8080 wasmedge target/wasm32-wasip1/release/http_server.wasm
```

## 動作確認

サーバーが起動したら、別のターミナルで以下のコマンドを実行してテストできます：

```shell
curl -X POST http://127.0.0.1:1234 -d "name=WasmEdge"
```

期待される応答：

```text
echo: name=WasmEdge
```

## Kubernetesでの実行

このサンプルはKubernetes上でも動作するように設計されています。
WASMEdgeランタイムが組み込まれたk3sクラスターでの実行が可能です。

### Dockerイメージの作成

```shell
cd server
docker build -t your-registry/wasm-http-server:latest .
docker push your-registry/wasm-http-server:latest
```

### Kubernetesマニフェストの例

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasm-http-server
spec:
  replicas: 3
  selector:
    matchLabels:
      app: wasm-http-server
  template:
    metadata:
      labels:
        app: wasm-http-server
    spec:
      runtimeClassName: wasmedge  # WASMEdgeランタイムクラスを指定
      containers:
      - name: http-server
        image: your-registry/wasm-http-server:latest
        ports:
        - containerPort: 1234
        env:
        - name: PORT
          value: "1234"
```

## ハッカソンでの利用

このサンプルは以下の用途で活用できます：

- **マイクロサービスアーキテクチャ**: 軽量なWASMサービスとして
- **エッジコンピューティング**: 高速起動が必要な環境で
- **コンテナ代替**: より安全で軽量なワークロードとして

## 技術スタック

### サーバー

- **言語**: Rust
- **HTTPライブラリ**: httpcodec, bytecodec
- **ネットワーク**: wasmedge_wasi_socket (v0.5.5)
- **ターゲット**: wasm32-wasip1
- **ランタイム**: WASMEdge

### クライアント

- **フレームワーク**: React
- **ビルドツール**: Vite
- **スタイリング**: CSS Modules / Tailwind CSS

## ライセンス

このプロジェクトはハッカソン用のサンプルコードです。
