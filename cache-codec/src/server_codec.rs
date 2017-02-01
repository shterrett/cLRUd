use std::io;
use std::str;
use tokio_core::io::{ Codec, EasyBuf };
use byteorder::{ ByteOrder, BigEndian };
use types::{ Command, CacheCommand, CacheResponse };
use helpers::parse_bytes;

pub struct CacheServerCodec {}

impl Codec for CacheServerCodec {
    type In = CacheCommand;
    type Out = CacheResponse;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if buf.len() == 0 { return Ok(None); }
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
        buf.extend(msg.response_type.as_bytes());
        buf.push(b'\n');

        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, msg.length);
        buf.extend(length.as_slice());
        buf.push(b'\n');

        buf.extend(msg.data);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio_core::io::{ Codec, EasyBuf };
    use byteorder::{ BigEndian, ByteOrder };
    use types::{ Command,
                 CommandResult,
                 CacheResponse
               };
    use super::CacheServerCodec;

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

        let mut decoder = CacheServerCodec {};
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

        let mut decoder = CacheServerCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.command, Command::GET);
        assert_eq!(decoded.key, "key".to_string());
        assert_eq!(decoded.length, 0);
        assert_eq!(decoded.value, vec![]);
    }

    #[test]
    fn encodes_success_result_with_payload() {
        let response_type = CommandResult::SUCCESS;
        let data: Vec<u8> = "cached data".to_string().as_bytes().to_vec();

        let response = CacheResponse {
            response_type: response_type,
            length: data.iter().len() as u64,
            data: data.clone()
        };

        let mut encoder = CacheServerCodec {};
        let mut encoded: Vec<u8> = vec![];
        let result = encoder.encode(response, &mut encoded);

        let mut expected = vec![];
        expected.extend("success".to_string().as_bytes());
        expected.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, data.iter().len() as u64);
        expected.extend(length);
        expected.push(b'\n');
        expected.extend(data);

        assert!(result.is_ok());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encodes_success_result_with_no_payload() {
        let response_type = CommandResult::SUCCESS;
        let data: Vec<u8> = vec![];

        let response = CacheResponse {
            response_type: response_type,
            length: 0 as u64,
            data: data.clone()
        };

        let mut encoder = CacheServerCodec {};
        let mut encoded: Vec<u8> = vec![];
        let result = encoder.encode(response, &mut encoded);

        let mut expected = vec![];
        expected.extend("success".to_string().as_bytes());
        expected.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, 0 as u64);
        expected.extend(length);
        expected.push(b'\n');
        expected.extend(data);

        assert!(result.is_ok());
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encodes_error_result() {
        let response_type = CommandResult::FAILURE;
        let data: Vec<u8> = "error message".to_string().as_bytes().to_vec();

        let response = CacheResponse {
            response_type: response_type,
            length: data.iter().len() as u64,
            data: data.clone()
        };

        let mut encoder = CacheServerCodec {};
        let mut encoded: Vec<u8> = vec![];
        let result = encoder.encode(response, &mut encoded);

        let mut expected = vec![];
        expected.extend("failure".to_string().as_bytes());
        expected.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, data.iter().len() as u64);
        expected.extend(length);
        expected.push(b'\n');
        expected.extend(data);

        assert!(result.is_ok());
        assert_eq!(encoded, expected);
    }
}

