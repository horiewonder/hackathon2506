# Simple HTTP Client

HTTPサーバーをテストするためのReact SPAクライアントです。指定したエンドポイントにPOSTします。

## 必要な環境

- Node.js (18.0.0 以降)
- npm または yarn
- モダンブラウザ (Chrome, Firefox, Safari, Edge)

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

1. **ブラウザでアクセス**: `http://localhost:5173` をブラウザで開く
2. **サーバーURLを確認**: デフォルト設定済み
3. **メッセージを入力**: 送信したいテキストを入力エリアに記入
4. **POST送信**: 「POST送信 📤」ボタンをクリック
5. **レスポンス確認**: サーバーからのエコーレスポンスが即座に表示されます

### サーバーURL設定

デフォルトでは本番のWASMサーバーに接続されますが、ローカル開発時は変更可能です：

### 送信データ形式

クライアントは以下の形式でデータを送信します：

- **Content-Type**: `text/plain`
- **HTTP Method**: POST
- **Body**: プレーンテキスト形式

## 動作確認

### ローカルサーバーとのテスト

```bash
# 1. サーバー側を起動（別ターミナル）
cd ../server
wasmedge target/wasm32-wasip1/release/http_server.wasm

# 2. クライアント側でサーバーURLを変更
# ブラウザでサーバーURLを http://localhost:1234 に変更

# 3. メッセージを送信してテスト
```

## ビルドとデプロイ

### プロダクションビルド

```bash
# ビルド実行
npm run build

# ビルド結果の確認
ls -la dist/

# ローカルでプレビュー
npm run preview
```

### 静的サイトデプロイ

ビルド結果（`dist/`フォルダ）を任意の静的サイトホスティングサービスにデプロイ：

```bash
# Netlify CLI使用例
npm install -g netlify-cli
netlify deploy --prod --dir=dist

# Vercel CLI使用例  
npm install -g vercel
vercel --prod

# AWS S3 + CloudFront例
aws s3 sync dist/ s3://your-bucket-name --delete
aws cloudfront create-invalidation --distribution-id YOUR_DISTRIBUTION_ID --paths "/*"
```

## トラブルシューティング

### CORS エラー

**症状**: ブラウザコンソールに CORS エラーが表示される

**原因**: サーバー側でCORSヘッダーが適切に設定されていない

**解決策**:

1. サーバー側のCORS設定を確認
2. ブラウザの開発者ツールでレスポンスヘッダーを確認

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

### ネットワークエラー

**症状**: 「ネットワークエラー」または「接続できません」

**解決策**:

1. サーバーが起動しているか確認
2. サーバーURLが正しいか確認
3. ファイアウォール設定を確認

### HTTP 400 Bad Request

**症状**: リクエストは送信されるが 400 エラーが返される

**解決策**:

1. Content-Type ヘッダーの確認
2. リクエストボディの形式確認
3. サーバーログの確認

```bash
# サーバーログ確認（Kubernetes環境）
kubectl logs -f deployment/wasm-http-deployment
```

### レスポンス表示の問題

**症状**: レスポンスが正しく表示されない

**解決策**:

1. ブラウザの開発者ツールでネットワークタブを確認
2. レスポンスのContent-Typeを確認
3. 文字エンコーディングの確認

## アーキテクチャ

```text
┌─────────────────┐    HTTP POST     ┌─────────────────┐
│   React Client  │ ───────────────► │  Traefik Ingress│
│  (localhost:5173│                  │                 │
└─────────────────┘                  └─────────────────┘
                                              │
                                              ▼
                                     ┌─────────────────┐
                                     │ Kubernetes Svc  │
                                     │   (ClusterIP)   │
                                     └─────────────────┘
                                              │
                                              ▼
                                     ┌─────────────────┐
                                     │ WASM HTTP Pods  │
                                     │ (ECR Private)   │
                                     └─────────────────┘
```

## 技術スタック

### フロントエンド

- **フレームワーク**: React 19.1.0
- **ビルドツール**: Vite 6.3.5
- **言語**: JavaScript (ES2022+)
- **HTTP通信**: Fetch API
- **スタイリング**: CSS3 (モダンなグラデーションとアニメーション)

### 開発・品質管理

- **リンター**: ESLint 9.25.0
- **開発サーバー**: Vite Dev Server (HMR対応)
- **ブラウザサポート**: ES2015+ 対応ブラウザ

### 本番環境

- **サーバー**: Kubernetes (k3s) + Traefik Ingress
- **プロトコル**: HTTP/1.1

## 開発用コマンド

```bash
# 開発サーバー起動（ホットリロード有効）
npm run dev

# プロダクションビルド
npm run build

# ビルド結果のローカルプレビュー
npm run preview

# コードリンティング
npm run lint

# 依存関係の更新確認
npm outdated

# 依存関係の脆弱性チェック
npm audit
```

## プロジェクト構成

```text
client/
├── public/
│   └── vite.svg
├── src/
│   ├── assets/
│   │   └── react.svg
│   ├── App.css          # メインスタイル
│   ├── App.jsx          # メインコンポーネント
│   ├── index.css        # グローバルスタイル
│   └── main.jsx         # エントリーポイント
├── index.html           # HTMLテンプレート
├── package.json         # 依存関係とスクリプト
├── vite.config.js       # Vite設定
├── eslint.config.js     # ESLint設定
└── README.md           # このファイル
```

## 関連リンク

- [サーバー側実装](../server/README.md)
- [Vite Documentation](https://vitejs.dev/)
- [React Documentation](https://react.dev/)
- [WebAssembly公式サイト](https://webassembly.org/)
