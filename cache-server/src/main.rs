extern crate clap;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate cache_codec;
extern crate lru_cache;

mod proto;
mod service;

use clap::{ Arg, App };
use std::sync::{ Arc, Mutex };
use tokio_proto::TcpServer;
use lru_cache::cache::LruCache;
use service::CacheSrv;
use proto::CacheCommandProto;

fn main() {
    let matches = App::new("CacheServer")
                      .version("001.0")
                      .author("Stuart <shterrett@gmail.com>")
                      .arg(Arg::with_name("address")
                           .help("ip address to listen on")
                           .short("a")
                           .long("address")
                           .takes_value(true))
                      .arg(Arg::with_name("port")
                           .help("port to listen on")
                           .short("p")
                           .long("port")
                           .takes_value(true))
                      .get_matches();


    let mut addr = matches.value_of("address").unwrap_or("0.0.0.0").to_string();
    let port = matches.value_of("port").unwrap_or("8080");
    addr.push_str(":");
    addr.push_str(port);

    let server = TcpServer::new(CacheCommandProto, addr.parse().unwrap());
    let cache = Arc::new(Mutex::new(LruCache::new(u64::pow(2, 9))));

    server.serve(move || Ok(
        CacheSrv {
            cache: cache.clone()
        }));
}
