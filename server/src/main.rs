use bytecodec::DecodeExt;
use httpcodec::{Request, RequestDecoder};
use std::io::{Read, Write};
use std::fmt;
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

// ログマクロ定義
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

macro_rules! log_warning {
    ($($arg:tt)*) => {
        println!("⚠️  [WARNING] {}", format!($($arg)*));
    };
}

// 統一エラーハンドリング
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

// HTTPレスポンス構造体
#[derive(Debug)]
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
    
    fn with_content_type(mut self, content_type: &str) -> Self {
        // Content-Typeが既に存在する場合は更新、なければ追加
        if let Some(pos) = self.headers.iter().position(|(key, _)| *key == "Content-Type") {
            self.headers[pos].1 = content_type.to_string();
        } else {
            self.headers.push(("Content-Type", content_type.to_string()));
        }
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

// サーバー設定構造体
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

// HTTPリクエスト処理
fn handle_http_request(req: Request<String>) -> HttpResponse {
    log_info!("Request: {}", req.method());
    
    let response_body = match req.method().as_str() {
        "GET" => {
            log_info!("Processing GET request");
            "GET request processed successfully".to_string()
        },
        "POST" => {
            log_info!("Processing POST request, body length: {}", req.body().len());
            format!("echo: {}", req.body())
        },
        _ => {
            log_warning!("Unexpected method in handle_http_request: {}", req.method());
            "Unexpected method".to_string()
        }
    };
    
    HttpResponse::new(200, "OK")
        .with_body(response_body)
        .with_content_type("text/plain")
}

// OPTIONSリクエスト処理（CORS プリフライト）
fn handle_options_request() -> HttpResponse {
    log_info!("CORS preflight request handled");
    
    HttpResponse::new(200, "OK")
        .with_content_type("text/plain")
}

// エラーレスポンス生成
fn create_error_response(error: ServerError) -> HttpResponse {
    match error {
        ServerError::RequestTooLarge => {
            log_error!("Request too large");
            HttpResponse::new(413, "Request Entity Too Large")
                .with_body("Request Entity Too Large".to_string())
                .with_content_type("text/plain")
        },
        ServerError::DecodingError(ref msg) => {
            log_error!("Request decode error: {}", msg);
            HttpResponse::new(400, "Bad Request")
                .with_body(format!("Bad Request: {}", msg))
                .with_content_type("text/plain")
        },
        ServerError::MethodNotAllowed => {
            log_error!("Method not allowed");
            HttpResponse::new(405, "Method Not Allowed")
                .with_body("Method Not Allowed".to_string())
                .with_content_type("text/plain")
        },
        ServerError::IoError(ref err) => {
            log_error!("IO error: {}", err);
            HttpResponse::new(500, "Internal Server Error")
                .with_body(format!("Internal Server Error: {}", err))
                .with_content_type("text/plain")
        }
    }
}

// クライアント接続処理（修正版3）
fn handle_client(mut stream: TcpStream, config: &ServerConfig) -> Result<(), ServerError> {
    let mut buff = vec![0u8; config.buffer_size];
    let mut data = Vec::new();
    let mut expected_total_length: Option<usize> = None;

    // HTTPリクエスト全体（ヘッダー + ボディ）を読み取り
    loop {
        let n = stream.read(&mut buff)?;
        if n == 0 {
            // 接続が閉じられた
            if expected_total_length.is_some() && data.len() < expected_total_length.unwrap() {
                // まだデータが不足している
                continue;
            }
            break;
        }
        
        data.extend_from_slice(&buff[0..n]);
        
        // リクエストサイズ制限をチェック
        if data.len() > config.max_request_size {
            let response = create_error_response(ServerError::RequestTooLarge);
            stream.write(response.to_string().as_bytes())?;
            stream.shutdown(Shutdown::Both)?;
            return Err(ServerError::RequestTooLarge);
        }
        
        // HTTPヘッダーの終端を検出（一回だけ実行）
        if expected_total_length.is_none() {
            if let Some(header_end) = data.windows(4).position(|window| window == b"\r\n\r\n") {
                let headers_str = String::from_utf8_lossy(&data[..header_end]);
                
                // Content-Lengthを探す
                let mut content_length = 0;
                for line in headers_str.lines() {
                    if line.to_lowercase().starts_with("content-length:") {
                        if let Some(length_str) = line.split(':').nth(1) {
                            content_length = length_str.trim().parse().unwrap_or(0);
                        }
                        break;
                    }
                }
                
                expected_total_length = Some(header_end + 4 + content_length);
                log_info!("Expected total HTTP request length: {} bytes (headers: {}, body: {})", 
                         expected_total_length.unwrap(), header_end + 4, content_length);
            }
        }
        
        // 必要なデータが揃ったかチェック
        if let Some(total_length) = expected_total_length {
            if data.len() >= total_length {
                log_info!("Complete HTTP request received: {} bytes", data.len());
                data.truncate(total_length);
                break;
            } else {
                log_info!("Partial HTTP request: {}/{} bytes", data.len(), total_length);
            }
        }
        
        // 無限ループ防止のため、バッファサイズが小さい場合も抜ける
        // ただし、expected_total_lengthが設定されている場合は続行
        if n < config.buffer_size && expected_total_length.is_none() {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let response = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => {
            // HTTPメソッドを明示的にチェック
            match req.method().as_str() {
                "GET" | "POST" => handle_http_request(req),
                "OPTIONS" => handle_options_request(),
                _ => create_error_response(ServerError::MethodNotAllowed),
            }
        },
        Err(e) => {
            log_error!("Decoder error with {} bytes: {:?}", data.len(), e);
            create_error_response(ServerError::DecodingError(format!("{:?}", e)))
        }
    };

    stream.write(response.to_string().as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let config = ServerConfig::from_env();
    
    log_info!("🚀 WASM HTTP Server starting...");
    log_info!("📋 Configuration:");
    log_info!("   - Port: {}", config.port);
    log_info!("   - Max request size: {}KB", config.max_request_size / 1024);
    log_info!("   - Buffer size: {}B", config.buffer_size);
    log_success!("✅ CORS enabled for all origins");
    log_success!("🔒 Security: Request size limit and method restrictions enabled");
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port), false)?;
    log_success!("🌐 Server listening on 0.0.0.0:{}", config.port);
    
    loop {
        match listener.accept(false) {
            Ok((stream, _addr)) => {
                match handle_client(stream, &config) {
                    Ok(_) => {
                        log_success!("Client request handled successfully");
                    },
                    Err(ServerError::RequestTooLarge) => {
                        log_warning!("Request rejected: too large");
                    },
                    Err(e) => {
                        log_error!("Client handling error: {}", e);
                    }
                }
            },
            Err(e) => {
                log_error!("Failed to accept connection: {}", e);
            }
        }
    }
}
