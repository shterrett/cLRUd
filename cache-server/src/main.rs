extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;

mod codec;
use codec::CacheCommandCodec;

fn main() {
    println!("Hello, world!");
}
