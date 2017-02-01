extern crate futures;
extern crate tokio_core;
extern crate cache_codec;

use std::io;
use std::net::{ ToSocketAddrs, SocketAddr };
use std::marker::Sync;
use tokio_core::io::{ Codec, EasyBuf };
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use futures::Future;
use cache_codec::types::{ CacheCommand, CacheResponse, Command };
use cache_codec::client_codec::CacheClientCodec;

pub trait Cacheable : Sync + Send {
    fn key(&self) -> String;
    fn value(&self) -> Vec<u8>;
    fn value_from_bytes(&self, val: Vec<u8>) -> Self;
}

pub struct CacheClient {
    address: SocketAddr
}

impl CacheClient {
    pub fn new(addr: String) -> Option<Self> {
        addr.to_socket_addrs()
            .ok()
            .and_then(|mut addresses| addresses.next())
            .map(|address| CacheClient { address: address })
    }

    pub fn get<T: Cacheable + 'static>(&self, item: T) -> io::Result<T> {
        let command = CacheCommand {
            command: Command::GET,
            key: item.key(),
            value: vec![],
            length: 0
        };

        self.send_request(command).map(move |response| item.value_from_bytes(response.data))
    }

    pub fn put<T: Cacheable + 'static>(&self, item: T) -> io::Result<T> {
        let value = item.value();
        let length = value.iter().len() as u64;
        let command = CacheCommand {
            command: Command::PUT,
            key: item.key(),
            value: value,
            length: length
        };

        self.send_request(command).map(move |response| item.value_from_bytes(response.data))
    }

    fn send_request(&self, cmd: CacheCommand) -> io::Result<CacheResponse> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let socket = TcpStream::connect(&self.address, &handle);

        let mut codec = CacheClientCodec {};
        let mut payload = vec![];
        let _ = codec.encode(cmd, &mut payload);

        core.run(
            socket.and_then(|socket| {
                tokio_core::io::write_all(socket, payload)
            }).and_then(|(socket, _)| {
                socket.shutdown(std::net::Shutdown::Write).expect("Couldn't shut down");
                tokio_core::io::read_to_end(socket, vec![])
            }).map(move |(_, data)| {
                codec.decode(&mut EasyBuf::from(data))
            }).and_then(|result| {
                result.and_then(|option| option.ok_or(io::Error::new(io::ErrorKind::Other, "no result")))
            })
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
