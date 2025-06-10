# WASM HTTP Server

WebAssembly（WASM）を使用したHTTPサーバーの実装です。
軽量で高速なWASIベースのHTTPサーバーとして動作し、Kubernetes環境での本格的な運用に対応しています。

## 概要

このHTTPサーバーは以下の特徴を持っています：

- **軽量**: WebAssemblyバイナリとして動作するため、非常に軽量です
- **高速起動**: 従来のコンテナと比較して起動時間が大幅に短縮されます
- **セキュリティ**: WASMのサンドボックス環境で実行されるため、セキュリティが向上します
- **ポータビリティ**: WASI（WebAssembly System Interface）を使用することで、様々な環境で動作可能です
- **堅牢なHTTP処理**: Content-Lengthベースの確実なリクエスト読み取りによる安定した通信

## 必要な環境

- Rust（2018 edition以降）
- rustup（wasm32-wasip1ターゲットの追加用）
- wasmedgeランタイム（実行用）
- Docker（コンテナ化用）

## セットアップ

### 1. WASMターゲットの追加

```shell
rustup target add wasm32-wasip1
```

### 2. サーバーのビルド

```shell
cargo build --target wasm32-wasip1 --release
```

## 実行方法

### ローカルでの実行

```shell
wasmedge target/wasm32-wasip1/release/http_server.wasm
```

デフォルトでは`localhost:1234`でサーバーが起動します。

### 環境変数での設定

以下の環境変数で動作を調整できます：

```shell
PORT=8080                    # ポート番号（デフォルト: 1234）
MAX_REQUEST_SIZE=16384       # 最大リクエストサイズ（デフォルト: 8192 bytes）
BUFFER_SIZE=4096            # バッファサイズ（デフォルト: 2048 bytes）
```

例：

```shell
PORT=8080 MAX_REQUEST_SIZE=16384 wasmedge target/wasm32-wasip1/release/http_server.wasm
```

## 動作確認

サーバーが起動したら、別のターミナルで以下のコマンドを実行してテストできます：

### GETリクエスト

```shell
curl http://localhost:1234
```

期待される応答：

```text
GET request processed successfully
```

### POSTリクエスト（プレーンテキスト）

```shell
curl -X POST http://localhost:1234 -d "Hello World"
```

期待される応答：

```text
echo: Hello World
```

### POSTリクエスト（フォームデータ）

```shell
curl -X POST http://localhost:1234 -H "Content-Type: application/x-www-form-urlencoded" -d "message=Hello%20World"
```

期待される応答：

```text
echo: message=Hello%20World
```

## Kubernetes（k3s）での実行

このサーバーはk3s環境での本格的な運用に対応しています。

### 前提条件

- k3s クラスター
- プライベートECRまたは任意のコンテナレジストリ
- AWS CLI（ECR使用時）

### 1. Dockerイメージの作成とプッシュ

```shell
# イメージをビルド
docker build -t your-account.dkr.ecr.region.amazonaws.com/wasm-http-server:latest .

# ECRにログイン
aws ecr get-login-password --region region | docker login --username AWS --password-stdin your-account.dkr.ecr.region.amazonaws.com

# イメージをプッシュ
docker push your-account.dkr.ecr.region.amazonaws.com/wasm-http-server:latest
```

### 2. ECRアクセス用のImagePullSecret作成

```shell
kubectl create secret docker-registry ecr-secret \
  --docker-server=your-account.dkr.ecr.region.amazonaws.com \
  --docker-username=AWS \
  --docker-password=$(aws ecr get-login-password --region region)
```

### 3. DeploymentとServiceのデプロイ

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: wasm-http-deployment
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
      imagePullSecrets:
      - name: ecr-secret
      containers:
      - name: http-server
        image: your-account.dkr.ecr.region.amazonaws.com/wasm-http-server:latest
        ports:
        - containerPort: 1234
        env:
        - name: PORT
          value: "1234"
---
apiVersion: v1
kind: Service
metadata:
  name: wasm-http-service
spec:
  selector:
    app: wasm-http-server
  ports:
  - port: 80
    targetPort: 1234
  type: ClusterIP
```

### 4. Ingress設定（Traefik使用）

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: wasm-http-ingress
spec:
  ingressClassName: traefik
  rules:
  - host: your-domain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: wasm-http-service
            port:
              number: 80
```

### 5. デプロイとアクセス確認

```shell
# マニフェストを適用
kubectl apply -f deployment.yaml
kubectl apply -f ingress.yaml

# Pod状況確認
kubectl get pods

# サービス確認
kubectl get svc

# Ingress確認
kubectl get ingress

# 動作確認（ドメイン設定後）
curl -X POST http://your-domain.com -d "Hello from Kubernetes"
```

## トラブルシューティング

### ErrImagePull エラー

ECRからのイメージ取得に失敗する場合：

1. インスタンスのIAMロールにECR権限があることを確認
2. ImagePullSecretが正しく設定されているか確認
3. イメージ名とタグが正確であることを確認

```shell
# 権限確認
aws sts get-caller-identity

# ECRリポジトリ確認
aws ecr describe-repositories

# ImagePullSecret再作成
kubectl delete secret ecr-secret
kubectl create secret docker-registry ecr-secret --docker-server=... --docker-username=AWS --docker-password=$(aws ecr get-login-password --region region)
```

### HTTP 400 Bad Request エラー

リクエスト処理に失敗する場合：

1. Pod のログを確認

```shell
kubectl logs -f deployment/wasm-http-deployment
```

2. Content-Type ヘッダーの確認

```shell
# プレーンテキストとして送信
curl -X POST http://your-domain.com -H "Content-Type: text/plain" -d "test message"
```

### Ingress アクセス不可

外部からアクセスできない場合：

1. DNS設定の確認
2. セキュリティグループで80番ポートが開放されているか確認
3. Traefik の動作確認

```shell
kubectl get pods -n kube-system | grep traefik
kubectl get svc -n kube-system | grep traefik
```

## アーキテクチャ

```text
[React Client] 
    ↓ HTTP POST
[Traefik Ingress] 
    ↓ k8s Service
[WASM HTTP Server Pods] 
    ↓ ECR Private Registry
[Docker Container with WASMEdge Runtime]
```

## 技術スタック

- **言語**: Rust
- **HTTPライブラリ**: httpcodec (v0.2.3), bytecodec (v0.4.15)
- **ネットワーク**: wasmedge_wasi_socket (v0.5.5)
- **ターゲット**: wasm32-wasip1
- **ランタイム**: WASMEdge
- **コンテナ**: Docker
- **オーケストレーション**: Kubernetes (k3s)
- **Ingress**: Traefik
- **レジストリ**: Amazon ECR

## 実装のポイント

### HTTPリクエスト処理の改良

このサーバーでは、TCPソケットレベルでのHTTPリクエスト処理において、以下の課題を解決しています：

1. **分割配信への対応**: HTTPリクエストがヘッダーとボディに分かれて届く場合の適切な処理
2. **Content-Length解析**: HTTPヘッダーからContent-Lengthを解析し、必要なデータ量を確実に読み取り
3. **タイミング問題の解決**: ネットワーク遅延による読み取りタイミング問題への対応

```rust
// HTTPリクエスト全体を確実に読み取るロジック
if let Some(header_end) = data.windows(4).position(|window| window == b"\r\n\r\n") {
    // Content-Lengthを解析
    let expected_total_length = header_end + 4 + content_length;
    
    // 必要なデータが揃うまで読み取り継続
    if data.len() >= expected_total_length {
        data.truncate(expected_total_length);
        break;
    }
}
```
