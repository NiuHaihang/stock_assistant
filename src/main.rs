use core::time;
use std::error::Error; // 使用 use 引入一个标准库的包，或者第三方的包

use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;

/// Rust 程序入口
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(test))
        .route("/robot", get(send_msg));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// 发起一个 HTTP 请求
async fn send_request(url: &str) -> Result<String, Box<dyn Error>> {
    let cli = reqwest::Client::new();
    let body = cli
        .get(url)
        .header("referer", "https://finance.sina.com.cn/")
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

async fn test() {
    let _res = get_stock_data().await.unwrap();
    ()
}

async fn get_stock_data() -> Result<(), Box<dyn Error>> {
    let url = "http://hq.sinajs.cn/list=sh600519";
    let response = send_request(url).await;

    match response {
        Ok(s) => {
            println!("response is:{}", s);
            Ok(())
        }
        Err(_e) => Ok(()),
    }
}

const WEBHOOK_URL: &str =
    "https://open.feishu.cn/open-apis/bot/v2/hook/13658ca3-74d9-4fff-abe7-2cfb8e0e0da2";
const WEBHOOK_SECRET: &str = "fGn4MgckPni3okA7zEtf8c";
use base64::encode;
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
// use ring::hmac;
use std::str;
use std::string::FromUtf8Error;
fn generate_signature(t: i64) -> Result<String, FromUtf8Error> {
    let str_to_sign = format!("{}\n{}", t.to_string(), WEBHOOK_SECRET);
    println!("{str_to_sign}");
    // let key = hmac::Key::new(hmac::HMAC_SHA256, WEBHOOK_SECRET.as_bytes());
    // let signature = hmac::sign(&key, str_to_sign.as_bytes());
    // let sig = encode(signature.as_ref().to_owned());

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(str_to_sign.as_bytes())
        .expect("Hmac can take key of any size!");
    mac.update(b"");
    let res = mac.finalize();

    let sig = encode(res.into_bytes());

    Ok(sig)
}
#[derive(Serialize, Deserialize)]
struct MsgContent {
    msg_type: String,
    time_stamp: String,
    sign: String,
    content: String,
}

async fn send_msg() -> String {
    let res = send_robot_msg().await;
    match res {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

async fn send_robot_msg() -> Result<String, Box<dyn Error>> {
    let now = Utc::now();
    println!("{:?}", now.to_rfc3339());
    let time_stamp = now.timestamp();
    let signature = generate_signature(time_stamp)?;

    println!("{signature}");

    let content = MsgContent {
        msg_type: "text".to_string(),
        time_stamp: time_stamp.to_string(),
        sign: signature,
        content: r#"{"text":"hello"}"#.to_string(),
    };
    // let mut map = HashMap::new();
    // map.insert("msg_type", "text".to_string());
    // map.insert("time_stamp", time_stamp.to_string());
    // map.insert("sign", signature);
    // map.insert("content", "hello".to_string());

    let cli = reqwest::Client::new();

    let content_str = serde_json::to_string(&content)?;
    let resp = cli
        .post(WEBHOOK_URL)
        .header("Content-Type", "application/json")
        .body(content_str)
        .send()
        .await?
        .text()
        .await?;
    println!("resp is:{}", resp);
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_send_robot_msg() {
        let res = send_robot_msg();
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }
}
