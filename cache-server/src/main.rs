extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate lru_cache;

mod codec;
mod proto;
mod service;

use std::sync::{ Arc, Mutex };
use tokio_proto::TcpServer;
use lru_cache::cache::LruCache;
use service::CacheSrv;
use proto::CacheCommandProto;

fn main() {
    let addr = "0.0.0.0:8080".parse().unwrap();

    let server = TcpServer::new(CacheCommandProto, addr);
    let cache = Arc::new(Mutex::new(LruCache::new(u64::pow(2, 9))));

    server.serve(move || Ok(
        CacheSrv {
            cache: cache.clone()
        }));
}
