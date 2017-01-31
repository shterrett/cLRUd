use std::io;
use tokio_core::io::{ Io, Framed };
use tokio_proto::pipeline::ServerProto;
use cache_codec::types::{ CacheCommand, CacheResponse };
use cache_codec::server_codec::CacheServerCodec;

pub struct CacheCommandProto;

impl<T: Io + 'static> ServerProto<T> for CacheCommandProto {
    type Request = CacheCommand;
    type Response = CacheResponse;
    type Transport = Framed<T, CacheServerCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CacheServerCodec {}))
    }
}
