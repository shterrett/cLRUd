extern crate clap;
extern crate cache_client;

use clap::{ Arg, App };
use std::io;
use std::io::{ BufReader, BufRead };
use cache_client::{ Cacheable, CacheClient };

struct CacheString {
    key: String,
    value: String
}

impl Cacheable for CacheString {
    fn key(&self) -> String {
        self.key.clone()
    }

    fn value(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }

    fn value_from_bytes(&self, val: Vec<u8>) -> Self {
        CacheString {
            key: self.key.clone(),
            value: String::from_utf8(val).unwrap()
        }
    }
}

fn process_line(line: String, client: &CacheClient) {
    let words = line.split_whitespace().fold(vec![], |mut cmd, word| {
        cmd.push(word);
        cmd
    });
    if words[0] == "get" {
        let cache_string = CacheString {
            key: words[1].to_string(),
            value: "".to_string()
        };
        let result = client.get(cache_string);
        match result {
            Ok(returned) => println!("Key: {}, Value: {}", returned.key, returned.value),
            Err(e) => println!("Error: {}", e)
        }
    } else if words[0] == "put" {
        let cache_string = CacheString {
            key: words[1].to_string(),
            value: words[2].to_string()
        };
        match client.put(cache_string) {
            Ok(returned) => println!("Key: {} cached", returned.key),
            Err(e) => println!("Error: {}", e)
        }
    } else {
        println!("invalid command: `put <key> <value>` or `get <key>`");
    }

}

fn main() {
    let matches = App::new("CacheClient")
                      .version("001.0")
                      .author("Stuart <shterrett@gmail.com>")
                      .arg(Arg::with_name("address")
                           .help("ip address to listen on")
                           .short("a")
                           .long("address")
                           .takes_value(true))
                      .arg(Arg::with_name("port")
                           .help("port to listen on")
                           .short("p")
                           .long("port")
                           .takes_value(true))
                      .get_matches();


    let mut addr = matches.value_of("address").unwrap_or("0.0.0.0").to_string();
    let port = matches.value_of("port").unwrap_or("8080");
    addr.push_str(":");
    addr.push_str(port);

    let client = CacheClient::new(addr).unwrap();

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    for res in reader.lines() {
        match res {
            Ok(line) => {
                process_line(line, &client);
            }
            Err(e) => println!("error: {}", e)
        }
    }
}
