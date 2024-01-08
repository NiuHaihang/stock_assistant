use core::time;
use std::error::Error; // 使用 use 引入一个标准库的包，或者第三方的包

use axum::{routing::get, Router};
use std::net::SocketAddr;

/// Rust 程序入口
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));

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

const WEBHOOK_URL: &str="https://open.feishu.cn/open-apis/bot/v2/hook/13658ca3-74d9-4fff-abe7-2cfb8e0e0da2";
const WEBHOOK_SECRET:&str="TgliN11mPBnZCZIQuKg39";
use chrono::prelude::*;
use ring::{hmac, signature};
use std::string::FromUtf8Error;
use std::str;
fn generate_signature()->Result<String,FromUtf8Error>{
    let now = Local::now();
    let time_stamp =now.timestamp();

    let mut str_to_sign = time_stamp.to_string()+"\n"+WEBHOOK_SECRET;
    
    let key = hmac::Key::new(hmac::HMAC_SHA256,WEBHOOK_SECRET.as_bytes());
    let signature = hmac::sign(&key, str_to_sign.as_bytes());
    String::from_utf8(signature.as_ref().to_vec())
}

async fn send_robot_msg()->Result<String,dyn Error>{
    let signature= 
}
