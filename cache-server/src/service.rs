use std::io;
use std::sync::{ Arc, Mutex };
use tokio_service::Service;
use futures::{ future, Future, BoxFuture };
use codec::{ Command, CommandResult, CacheCommand, CacheResponse };
use lru_cache::cache::LruCache;

pub struct CacheSrv {
    pub cache: Arc<Mutex<LruCache<String>>>
}

impl Service for CacheSrv {
    type Request = CacheCommand;
    type Response = CacheResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req.command {
            Command::PUT => {
                self.cache.lock().unwrap().put(req.key, req.value);
                let response = CacheResponse {
                    response_type: CommandResult::SUCCESS,
                    length: 0,
                    data: vec![]
                };
                future::ok(response).boxed()
            },
            Command::GET => {
                match self.cache.lock().unwrap().get(&req.key) {
                    Some(data) => {
                        let response = CacheResponse {
                            response_type: CommandResult::SUCCESS,
                            length: data.iter().len() as u64,
                            data: data.clone()
                        };
                        future::ok(response).boxed()
                    },
                    None => {
                        let msg = "Not Found".to_string().as_bytes().to_vec();
                        let response = CacheResponse {
                            response_type: CommandResult::FAILURE,
                            length: msg.iter().len() as u64,
                            data: msg
                        };
                        future::ok(response).boxed()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::{ Arc, Mutex };
    use futures::Future;
    use tokio_service::Service;
    use lru_cache::cache::LruCache;
    use codec::{ Command, CommandResult, CacheCommand };
    use super::{ CacheSrv };

    #[test]
    fn test_puts_in_cache() {
        let value = "message".to_string().as_bytes().to_vec();
        let cache = LruCache::new(8);
        let service = CacheSrv {
            cache: Arc::new(Mutex::new(cache))
        };
        let request = CacheCommand {
            command: Command::PUT,
            key: "key".to_string(),
            value: value.clone(),
            length: value.iter().len() as u64
        };

        match service.call(request).wait() {
            Ok(response) => {
                assert_eq!(response.response_type, CommandResult::SUCCESS);
                assert_eq!(response.length, 0);
                assert_eq!(response.data, vec![]);
                assert_eq!(service.cache.lock().unwrap().get(&"key".to_string()),
                           Some(&value)
                          );
            },
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn test_gets_from_cache() {
        let key = "key".to_string();
        let value = "message".to_string().as_bytes().to_vec();
        let cache = Arc::new(Mutex::new(LruCache::new(8)));
        let service = CacheSrv { cache: cache.clone() };
        service.cache.lock().unwrap().put(key.clone(), value.clone());

        let request = CacheCommand {
            command: Command::GET,
            key: key.clone(),
            value: vec![],
            length: 0
        };

        match service.call(request).wait() {
            Ok(response) => {
                assert_eq!(response.response_type, CommandResult::SUCCESS);
                assert_eq!(response.length, value.iter().len() as u64);
                assert_eq!(response.data, value);
            },
            Err(e) => {
                panic!(e);
            }
        }
    }

    #[test]
    fn test_get_not_present() {
        let cache = Arc::new(Mutex::new(LruCache::new(8)));
        let service = CacheSrv { cache: cache.clone() };

        let request = CacheCommand {
            command: Command::GET,
            key: "key".to_string(),
            value: vec![],
            length: 0
        };

        let msg = "Not Found".to_string().as_bytes().to_vec();
        match service.call(request).wait() {
            Ok(response) => {
                assert_eq!(response.response_type, CommandResult::FAILURE);
                assert_eq!(response.length, msg.iter().len() as u64);
                assert_eq!(response.data, msg);
            },
            Err(e) => {
                panic!(e);
            }
        }
    }
}
