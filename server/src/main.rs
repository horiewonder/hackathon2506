use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

fn handle_http(req: Request<String>) -> String {
    let body = format!("echo: {}", req.body());
    
    // 手動でHTTPレスポンスを作成（CORSヘッダー付き）
    format!(
        "HTTP/1.1 200 OK\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    )
}

fn handle_options() -> String {
    // OPTIONSリクエスト（プリフライト）用のレスポンス
    format!(
        "HTTP/1.1 200 OK\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Content-Length: 0\r\n\
         \r\n"
    )
}

fn handle_error(error_msg: &str) -> String {
    let body = format!("Error: {}", error_msg);
    format!(
        "HTTP/1.1 500 Internal Server Error\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    )
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let response = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => {
            // OPTIONSリクエストかどうかチェック
            if req.method().as_str() == "OPTIONS" {
                handle_options()
            } else {
                handle_http(req)
            }
        },
        Err(e) => {
            handle_error(&format!("{:?}", e))
        }
    };

    stream.write(response.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("🚀 WASM HTTP Server starting on port {}", port);
    println!("✅ CORS enabled for all origins");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
