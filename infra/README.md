# WasmEdge + K3s セットアップガイド 🚀

このガイドでは、K3s上でWasmEdgeランタイムを使ってWebAssemblyコンテナを実行する環境を構築する手順を説明します。

## 📋 前提条件

- Ubuntu 24.04 LTS (arm64環境)
- sudo権限
- インターネット接続

## 🔧 ステップ1: WasmEdgeのインストール

```bash
# WasmEdgeランタイムをインストール
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local
# インストール確認
wasmedge --version
```

## 🐳 ステップ2: K3sのインストール

```bash
# K3sをインストール（containerdがデフォルトで有効）
curl -sfL https://get.k3s.io | sh -

# kubectlのセットアップ
sudo chmod 644 /etc/rancher/k3s/k3s.yaml
export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
```

## 🛠 ステップ3: containerd-shim-wasmedgeの構築

### 依存関係のインストール

```bash
sudo apt-get update
sudo apt-get install -y pkg-config libsystemd-dev libdbus-glib-1-dev \
    build-essential libelf-dev libseccomp-dev libclang-dev libssl-dev \
    protobuf-compiler

# Rustのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### runwasiのビルドとインストール

```bash
cd /tmp
git clone https://github.com/containerd/runwasi.git
cd runwasi

# WasmEdge shimをビルド
make build-wasmedge

# システムにインストール（arm64環境の場合）
sudo install -D -m 755 ./target/aarch64-unknown-linux-gnu/debug/containerd-shim-wasmedge-v1 /usr/local/bin/containerd-shim-wasmedge-v1

# 確認
ls -la /usr/local/bin/containerd-shim-wasmedge-v1
```

## ⚙️ ステップ4: containerd設定

```bash
# K3s用のcontainerd設定テンプレートを作成
sudo tee /var/lib/rancher/k3s/agent/etc/containerd/config.toml.tmpl > /dev/null <<'EOF'
{{ template "base" . }}

[plugins.'io.containerd.cri.v1.runtime'.containerd.runtimes.wasmedge]
  runtime_type = "io.containerd.wasmedge.v1"
EOF

# K3sを再起動
sudo systemctl restart k3s
```

## 🎯 ステップ5: RuntimeClassの作成

```bash
kubectl apply -f - <<EOF
apiVersion: node.k8s.io/v1
kind: RuntimeClass
metadata:
  name: wasmedge
handler: wasmedge
EOF

# 確認
kubectl get runtimeclass
```

## 🧪 ステップ6: Wasmアプリケーションのテスト

### サンプルアプリのデプロイ

```bash
kubectl apply -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: wasm-demo
spec:
  runtimeClassName: wasmedge
  containers:
  - name: wasm
    image: wasmedge/example-wasi:latest
    command: ["/wasi_example_main.wasm", "50000000"]
EOF
```

### 実行結果の確認

```bash
# Pod状態確認
kubectl get pods

# ログ確認
kubectl logs wasm-demo

# 詳細情報確認
kubectl describe pod wasm-demo
```

## 🌐 ステップ7: HTTPサーバーアプリのテスト

```bash
kubectl apply -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: wasm-http-demo
  labels:
    app: wasm-http
spec:
  runtimeClassName: wasmedge
  containers:
  - name: wasm-http
    image: wasmedge/example-wasi-http:latest
    ports:
    - containerPort: 1234
---
apiVersion: v1
kind: Service
metadata:
  name: wasm-http-service
spec:
  selector:
    app: wasm-http
  ports:
  - port: 80
    targetPort: 1234
  type: NodePort
EOF

# テスト
kubectl port-forward pod/wasm-http-demo 8080:1234
# 別ターミナルで: curl http://localhost:8080
```

## ✅ 成功の確認ポイント

1. **Pod Status**: `Running` または `Completed`（バッチ処理の場合）
1. **Container ID**: `containerd://` で始まる
1. **Exit Code**: `0`（正常終了）
1. **ログ出力**: WASIの機能（乱数、ファイルアクセス、環境変数など）が動作
1. **Kubernetes統合**: 環境変数にKubernetesサービス情報が含まれる

## 🎉 期待される出力例

```text
Random number: -1459975930
Random bytes: [70, 13, 164, ...]
Printed from wasi: This is from a main function
The env vars are as follows.
PATH: /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
KUBERNETES_SERVICE_HOST: 10.43.0.1
KUBERNETES_SERVICE_PORT: 443
...
```

## 🚨 トラブルシューティング

### containerd設定が反映されない場合

```bash
# k3s再起動
sudo systemctl restart k3s

# containerd設定確認
sudo cat /var/lib/rancher/k3s/agent/etc/containerd/config.toml
```

### “no runtime for wasmedge is configured” エラー

- containerd設定テンプレートが正しく作成されているか確認
- containerd-shim-wasmedge-v1が正しくインストールされているか確認
