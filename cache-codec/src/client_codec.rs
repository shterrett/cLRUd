use std::io;
use tokio_core::io::{ Codec, EasyBuf };
use byteorder::{ ByteOrder, BigEndian };
use types::{ CacheCommand, CommandResult, CacheResponse };
use helpers::{ parse_bytes, encode_int };

pub struct CacheClientCodec {}

impl Codec for CacheClientCodec {
    type In = CacheResponse;
    type Out = CacheCommand;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        let response_type = parse_bytes(buf, |bytes| CommandResult::from_bytes(bytes));
        let length = parse_bytes(buf, |bytes| Some(BigEndian::read_u64(bytes)));
        let mut data: Vec<u8> = vec![];

        if let (Some(response), Some(l)) = (response_type, length) {
            data.extend_from_slice(buf.drain_to(l as usize).as_slice());
            Ok(Some(CacheResponse {
                response_type: response,
                length: l,
                data: data
            }))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "invalid response"))
        }
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(msg.command.as_bytes());
        buf.push(b'\n');

        buf.extend(msg.key.as_bytes());
        buf.push(b'\n');

        let length = encode_int(msg.length);
        buf.extend(length.as_slice());
        buf.push(b'\n');

        buf.extend(msg.value);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio_core::io::{ Codec, EasyBuf };
    use byteorder::{ BigEndian, ByteOrder };
    use types::{ Command,
                 CacheCommand,
                 CommandResult
               };
    use super::CacheClientCodec;

    #[test]
    fn encodes_put_command() {
        let key = "key".to_string();
        let value = "value".to_string().into_bytes();
        let length = value.iter().len() as u64;
        let mut length_as_bytes = vec![0; 8];
        BigEndian::write_u64(&mut length_as_bytes, length);

        let command = CacheCommand {
            key: key.clone(),
            value: value.clone(),
            length: length,
            command: Command::PUT
        };

        let mut bytes = vec![];
        let mut encoder = CacheClientCodec {};
        let result = encoder.encode(command, &mut bytes);

        let mut expected = vec![];
        expected.extend("put".to_string().into_bytes());
        expected.push(b'\n');
        expected.extend(key.into_bytes());
        expected.push(b'\n');
        expected.extend(length_as_bytes);
        expected.push(b'\n');
        expected.extend(value);

        assert!(result.is_ok());
        assert_eq!(bytes, expected);
    }

    #[test]
    fn encodes_get_command() {
        let key = "key".to_string();
        let length = 0 as u64;
        let mut length_as_bytes = vec![0; 8];
        BigEndian::write_u64(&mut length_as_bytes, length);

        let command = CacheCommand {
            key: key.clone(),
            value: vec![],
            length: length,
            command: Command::GET
        };

        let mut bytes = vec![];
        let mut encoder = CacheClientCodec {};
        let result = encoder.encode(command, &mut bytes);

        let mut expected = vec![];
        expected.extend("get".to_string().into_bytes());
        expected.push(b'\n');
        expected.extend(key.into_bytes());
        expected.push(b'\n');
        expected.extend(length_as_bytes);
        expected.push(b'\n');

        assert!(result.is_ok());
        assert_eq!(bytes, expected);
    }

    #[test]
    fn decodes_success_result_with_payload() {
        let response_type = CommandResult::SUCCESS;
        let data: Vec<u8> = "cached data".to_string().as_bytes().to_vec();

        let mut bytes = vec![];
        bytes.extend("success".to_string().as_bytes());
        bytes.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, data.iter().len() as u64);
        bytes.extend(length);
        bytes.push(b'\n');
        bytes.extend(data.clone());

        let mut decoder = CacheClientCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.response_type, response_type);
        assert_eq!(decoded.length, data.iter().len() as u64);
        assert_eq!(decoded.data, data);
    }

    #[test]
    fn decodes_success_result_with_no_payload() {
        let response_type = CommandResult::SUCCESS;
        let data: Vec<u8> = vec![];

        let mut bytes = vec![];
        bytes.extend("success".to_string().as_bytes());
        bytes.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, data.iter().len() as u64);
        bytes.extend(length);
        bytes.push(b'\n');
        bytes.extend(data.clone());

        let mut decoder = CacheClientCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.response_type, response_type);
        assert_eq!(decoded.length, data.iter().len() as u64);
        assert_eq!(decoded.data, data.clone());
    }

    #[test]
    fn decodes_error_result() {
        let response_type = CommandResult::FAILURE;
        let data: Vec<u8> = vec![];

        let mut bytes = vec![];
        bytes.extend("failure".to_string().as_bytes());
        bytes.push(b'\n');
        let mut length = vec![0; 8];
        BigEndian::write_u64(&mut length, data.iter().len() as u64);
        bytes.extend(length);
        bytes.push(b'\n');
        bytes.extend(data.clone());

        let mut decoder = CacheClientCodec {};
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.response_type, response_type);
        assert_eq!(decoded.length, data.iter().len() as u64);
        assert_eq!(decoded.data, data);
    }
}
