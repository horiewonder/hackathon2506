# クレートバージョンアップ計画とリファクタリング提案

## 📊 現在のクレート状況

| クレート | 現在のバージョン | 最新バージョン | アップデート可否 | 優先度 |
|---------|-----------------|---------------|----------------|--------|
| bytecodec | 0.4.15 | 0.5.0 | ✅ 可能 | 🔴 高 |
| httpcodec | 0.2.3 | 0.2.3 | ✅ 最新 | 🟢 低 |
| wasmedge_wasi_socket | 0.5.5 | 0.5.5 | ✅ 最新 | 🟢 低 |

## 🔄 バージョンアップ詳細

### 1. bytecodec (0.4.15 → 0.5.0)

**主な変更点:**
- `trackable` クレートをv1にアップデート
- lint警告の修正
- コードの最適化とクリーンアップ

**破壊的変更:**
- trackableのAPIが変更されている可能性
- エラーハンドリングの方法が変更される可能性

**アップデート手順:**
1. `Cargo.toml`の`bytecodec`を`"0.5.0"`に更新
2. コンパイルエラーの確認と修正
3. テストの実行と動作確認

### 2. httpcodec (0.2.3)
- 最新版を使用中
- 6年間更新されていないが、安定している

### 3. wasmedge_wasi_socket (0.5.5)
- 最新版を使用中
- 9ヶ月前の更新で比較的新しい

## 🛠️ リファクタリング提案

### 1. エラーハンドリングの改善

**現在の問題:**
- エラーメッセージが散在している
- 一貫性のないエラー処理

**提案:**
```rust
use std::fmt;

#[derive(Debug)]
enum ServerError {
    RequestTooLarge,
    DecodingError(String),
    IoError(std::io::Error),
    MethodNotAllowed,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::RequestTooLarge => write!(f, "Request entity too large"),
            ServerError::DecodingError(msg) => write!(f, "Request decode error: {}", msg),
            ServerError::IoError(err) => write!(f, "IO error: {}", err),
            ServerError::MethodNotAllowed => write!(f, "Method not allowed"),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IoError(err)
    }
}
```

### 2. HTTP レスポンス生成の統一化

**現在の問題:**
- レスポンス生成コードが重複している
- CORS ヘッダーが各関数で重複

**提案:**
```rust
struct HttpResponse {
    status_code: u16,
    status_text: &'static str,
    headers: Vec<(&'static str, String)>,
    body: String,
}

impl HttpResponse {
    fn new(status_code: u16, status_text: &'static str) -> Self {
        Self {
            status_code,
            status_text,
            headers: vec![
                ("Access-Control-Allow-Origin", "*".to_string()),
                ("Access-Control-Allow-Methods", "GET, POST, OPTIONS".to_string()),
                ("Access-Control-Allow-Headers", "Content-Type".to_string()),
            ],
            body: String::new(),
        }
    }
    
    fn with_body(mut self, body: String) -> Self {
        self.headers.push(("Content-Length", body.len().to_string()));
        self.headers.push(("Content-Type", "text/plain".to_string()));
        self.body = body;
        self
    }
    
    fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
```

### 3. 設定の外部化

**現在の問題:**
- 定数がハードコードされている
- 設定変更時にリコンパイルが必要

**提案:**
```rust
#[derive(Debug)]
struct ServerConfig {
    max_request_size: usize,
    buffer_size: usize,
    port: String,
}

impl ServerConfig {
    fn from_env() -> Self {
        Self {
            max_request_size: std::env::var("MAX_REQUEST_SIZE")
                .unwrap_or("8192".to_string())
                .parse()
                .unwrap_or(8 * 1024),
            buffer_size: std::env::var("BUFFER_SIZE")
                .unwrap_or("2048".to_string())
                .parse()
                .unwrap_or(2048),
            port: std::env::var("PORT").unwrap_or("1234".to_string()),
        }
    }
}
```

### 4. ログ機能の強化

**提案:**
```rust
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("ℹ️  [INFO] {}", format!($($arg)*));
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!("❌ [ERROR] {}", format!($($arg)*));
    };
}

macro_rules! log_success {
    ($($arg:tt)*) => {
        println!("✅ [SUCCESS] {}", format!($($arg)*));
    };
}
```

## 📋 実装計画

### フェーズ1: バージョンアップ (優先度: 高)
1. `bytecodec`を0.5.0にアップデート
2. コンパイルエラーの修正
3. 動作テスト

### フェーズ2: リファクタリング (優先度: 中)
1. エラーハンドリングの統一
2. HTTPレスポンス生成の改善
3. 設定の外部化

### フェーズ3: 機能拡張 (優先度: 低)
1. ログ機能の強化
2. パフォーマンス監視
3. ヘルスチェック機能

## ⚠️ 注意点

1. **WASM環境でのテスト**: 変更後は必ずWASMエッジ環境でのテストが必要
2. **後方互換性**: 既存のクライアントコードに影響がないか確認
3. **パフォーマンス**: リファクタリング後のパフォーマンス影響を測定

## 🎯 期待される効果

1. **保守性の向上**: エラーハンドリングとコード構造の統一
2. **拡張性の向上**: 設定の外部化により柔軟な運用が可能
3. **安定性の向上**: 最新のクレートによるバグ修正とセキュリティ向上
4. **開発効率の向上**: ログ機能とエラー処理の改善
