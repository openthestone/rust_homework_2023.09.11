use lazy_static::lazy_static;
use std::net::SocketAddr;
use mini_redis::LogLayer;
use volo::FastStr;
use volo_gen::mini_redis::{RedisRequest,RequestType,};
use std::sync::Arc;

lazy_static! {
    static ref CLIENT: volo_gen::mini_redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini_redis::RedisServiceClientBuilder::new("redis")
            .layer_outer(LogLayer)
            // .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();

    // let args = std::env::args().collect::<Vec<String>>();
    // build a vec of static str

    let req = match args[1].to_lowercase().as_str() {
        "set" => {
            RedisRequest{
                key: Some(FastStr::from(Arc::new(args.remove(2)))),
                value: Some(FastStr::from(Arc::new(args.remove(2)))),
                request_type: RequestType::Set,
                expire_time: None,
            }
        },
        "get" => {
            RedisRequest{
                key: Some(FastStr::from(Arc::new(args.remove(2)))),
                value: None,
                request_type: RequestType::Get,
                expire_time: None,
            }
        },
        "del" => {
            RedisRequest{
                key: Some(FastStr::from(Arc::new(args.remove(2)))),
                value: None,
                request_type: RequestType::Del,
                expire_time: None,
            }
        },
        "ping" => {
            RedisRequest{
                key: None,
                value: None,
                request_type: RequestType::Ping,
                expire_time: None,
            }
        },
        "subscribe" => {
            RedisRequest{
                key: Some(FastStr::from(Arc::new(args.remove(2)))),
                value: None,
                request_type: RequestType::Subscribe,
                expire_time: None,
            }
        },
        "publish" => {
            RedisRequest{
                key: Some(FastStr::from(Arc::new(args.remove(2)))),
                value: None,
                request_type: RequestType::Publish,
                expire_time: None,
            }
        },
        _ => {
            panic!("unknown command");
        }
    };
    // let req = RedisRequest{
    //     key: Some(FastStr::from("zju")),
    //     value: Some(FastStr::from("114514")),
    //     request_type: volo_gen::mini_redis::RequestType::Set,
    //     expire_time: None,
    // };
    // let req = RedisRequest{
    //     key: Some(FastStr::from("zju")),
    //     value: None,
    //     request_type: volo_gen::mini_redis::RequestType::Get,
    //     expire_time: None,
    // };
    let resp = CLIENT.redis_command(req).await;
    match resp {
        Ok(info) => tracing::info!("{:?}", info.value),
        Err(e) => tracing::error!("{:?}", e),
    }
}


