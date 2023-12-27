use std::error::Error; // 使用 use 引入一个标准库的包，或者第三方的包

use axum::{routing::get, Router};
use clap::Parser; // clap 是一个 Rust 社区开发的命令行参数解析库
use reqwest::blocking::Client; // reqwest 是一个 Rust 社区开发的 HTTP 客户端库
use reqwest::header::HeaderMap;
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
