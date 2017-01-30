use std::io;
use tokio_core::io::{ Io, Framed };
use tokio_proto::pipeline::ServerProto;
use codec::{ CacheCommand, CacheCommandCodec };

pub struct CacheCommandProto;

impl<T: Io + 'static> ServerProto<T> for CacheCommandProto {
    type Request = CacheCommand;
    type Response = String;
    type Transport = Framed<T, CacheCommandCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CacheCommandCodec {}))
    }
}
