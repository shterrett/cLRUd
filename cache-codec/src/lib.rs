extern crate tokio_core;
extern crate byteorder;

pub mod types;
mod helpers;
pub mod server_codec;
pub mod client_codec;

#[cfg(test)]
mod test {
    use tokio_core::io::{ Codec, EasyBuf };
    use types::{ Command,
                 CacheCommand,
                 CommandResult,
                 CacheResponse
               };
    use client_codec::CacheClientCodec;
    use server_codec::CacheServerCodec;

    #[test]
    fn cache_command_symmetry() {
        let command = CacheCommand {
            command: Command::PUT,
            key: "key".to_string(),
            value: "value".to_string().as_bytes().to_vec(),
            length: "value".to_string().into_bytes().iter().len() as u64
        };

        let mut encoder = CacheClientCodec {};
        let mut decoder = CacheServerCodec {};
        let mut bytes = vec![];

        let _ = encoder.encode(command, &mut bytes);
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.command, Command::PUT);
        assert_eq!(decoded.key, "key".to_string());
        assert_eq!(decoded.value, "value".to_string().as_bytes().to_vec());
        assert_eq!(decoded.length, "value".to_string().into_bytes().iter().len() as u64);

    }

    #[test]
    fn cache_result_symmetry() {
        let response = CacheResponse {
            response_type: CommandResult::SUCCESS,
            data: "cached_data".to_string().as_bytes().to_vec(),
            length: "cached_data".to_string().as_bytes().iter().len() as u64
        };

        let mut encoder = CacheServerCodec {};
        let mut decoder = CacheClientCodec {};
        let mut bytes = vec![];

        let _ = encoder.encode(response, &mut bytes);
        let result = decoder.decode(&mut EasyBuf::from(bytes));

        let decoded = result.unwrap().unwrap();
        assert_eq!(decoded.response_type, CommandResult::SUCCESS);
        assert_eq!(decoded.data, "cached_data".to_string().as_bytes().to_vec());
        assert_eq!(decoded.length, "cached_data".to_string().as_bytes().iter().len() as u64);

    }
}
