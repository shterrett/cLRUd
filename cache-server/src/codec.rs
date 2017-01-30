use std::io;
use std::str;
use tokio_core::io::{ Codec, EasyBuf };
use byteorder::{ ByteOrder, BigEndian };

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    PUT,
    GET
}

impl Command {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        str::from_utf8(&bytes)
                .ok()
                .and_then(|command| {
                    if command == "put" {
                        Some(Command::PUT)
                    } else if command == "get" {
                        Some(Command::GET)
                    } else {
                        None
                    }
                })
    }
}

#[derive(Debug)]
pub struct CacheCommand {
    command: Command,
    key: String,
    length: u64,
    value: Vec<u8>
}

pub struct CacheCommandCodec {}

impl Codec for CacheCommandCodec {
    type In = CacheCommand;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let command = parse_bytes(buf, |bytes| Command::from_bytes(bytes));
        let key = parse_bytes(buf, |bytes| str::from_utf8(bytes).ok().map(|s| s.to_string()));
        let length = parse_bytes(buf, |bytes| Some(BigEndian::read_u64(bytes)));
        let mut value: Vec<u8> = vec![];


        if let (Some(cmd), Some(k), Some(l)) = (command, key, length) {
            value.extend_from_slice(buf.drain_to(l as usize).as_slice());
            Ok(Some(CacheCommand {
                        command: cmd,
                        key: k,
                        length: l,
                        value: value
                    }))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "invalid request"))
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

fn parse_bytes<F, T>(buf: &mut EasyBuf, convert: F) -> Option<T>
    where F: Fn(&[u8]) -> Option<T> {
    buf.as_slice().iter().position(|&b| b == b'\n').and_then(|idx| {
        let bytes = buf.drain_to(idx);
        buf.drain_to(1);
        convert(bytes.as_slice())
    })
}



#[cfg(test)]
mod test {
    use tokio_core::io::{ Codec, EasyBuf };
    use byteorder::{ BigEndian, ByteOrder };
    use super::{ CacheCommandCodec, Command };

    #[test]
    fn decodes_put_command_with_value() {
        let command = "put";
        let key = "key";
        let value = "value".to_string().into_bytes();
        let length = value.iter().len() as u64;
        let mut length_as_bytes = vec![0; 8];
        BigEndian::write_u64(&mut length_as_bytes, length);

        let mut bytes = vec![];
        bytes.extend(command.to_string().into_bytes());
        bytes.push(b'\n');
        bytes.extend(key.to_string().into_bytes());
        bytes.push(b'\n');
        bytes.extend(length_as_bytes);
        bytes.push(b'\n');
        bytes.extend(value);

        let mut decoder = CacheCommandCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.command, Command::PUT);
        assert_eq!(decoded.key, "key".to_string());
        assert_eq!(decoded.length, 5);
        assert_eq!(decoded.value, "value".to_string().into_bytes());
    }

    #[test]
    fn decodes_get_command() {
        let command = "get";
        let key = "key";
        let length = 0 as u64;
        let mut length_as_bytes = vec![0; 8];
        BigEndian::write_u64(&mut length_as_bytes, length);

        let mut bytes = vec![];
        bytes.extend(command.to_string().into_bytes());
        bytes.push(b'\n');
        bytes.extend(key.to_string().into_bytes());
        bytes.push(b'\n');
        bytes.extend(length_as_bytes);
        bytes.push(b'\n');

        let mut decoder = CacheCommandCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.command, Command::GET);
        assert_eq!(decoded.key, "key".to_string());
        assert_eq!(decoded.length, 0);
        assert_eq!(decoded.value, vec![]);
    }
}
