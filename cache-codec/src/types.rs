use std::str;

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    PUT,
    GET
}

impl Command {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
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

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            &Command::PUT => "put".to_string().as_bytes().to_vec(),
            &Command::GET => "get".to_string().as_bytes().to_vec()
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CommandResult {
    SUCCESS,
    FAILURE
}

impl CommandResult {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            &CommandResult::SUCCESS => "success".to_string().as_bytes().to_vec(),
            &CommandResult::FAILURE => "failure".to_string().as_bytes().to_vec()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        str::from_utf8(&bytes)
                .ok()
                .and_then(|result| {
                    if result == "success" {
                        Some(CommandResult::SUCCESS)
                    } else if result == "failure" {
                        Some(CommandResult::FAILURE)
                    } else {
                        None
                    }
                })
    }
}

#[derive(Debug)]
pub struct CacheCommand {
    pub command: Command,
    pub key: String,
    pub length: u64,
    pub value: Vec<u8>
}

#[derive(Debug)]
pub struct CacheResponse {
    pub response_type: CommandResult,
    pub length: u64,
    pub data: Vec<u8>
}
