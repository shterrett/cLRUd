use byteorder::{ ByteOrder, BigEndian };
use tokio_core::io::EasyBuf;

pub fn parse_bytes<F, T>(buf: &mut EasyBuf, convert: F) -> Option<T>
    where F: Fn(&[u8]) -> Option<T> {
    buf.as_slice().iter().position(|&b| b == b'\n').and_then(|idx| {
        let bytes = buf.drain_to(idx);
        buf.drain_to(1);
        convert(bytes.as_slice())
    })
}

pub fn encode_int(i: u64) -> Vec<u8> {
    let mut length = vec![0; 8];
    BigEndian::write_u64(&mut length, i);
    return length;
}
