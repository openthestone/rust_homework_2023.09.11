#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use mini_redis::LogLayer;
use mini_redis::{S};
use std::sync::Mutex;
use std::collections::HashMap;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);
    volo_gen::mini_redis::RedisServiceServer::new(S{map: Mutex::new(HashMap::<String,String>::new())})
        .layer_front(LogLayer)
        // .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
