// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use kvd_core::{Engine, Value};
use std::io::BufRead;
use std::sync::Arc;
use std::sync::RwLock;
use std::{io::BufReader, net::TcpStream};

enum RespType {
    SimpleStrings(Option<WaitingCommands<String>>),
    SimpleErrors(Option<WaitingCommands<String>>),
    Integers(Option<WaitingCommands<u64>>),
    BulkStrings(Option<WaitingCommands<String>>),
    Arrays(Option<WaitingCommands<Vec<String>>>),
    Nulls(Option<WaitingCommands<char>>),
    Booleans(Option<WaitingCommands<char>>),
}

struct WaitingCommands<T> {
    done: bool,
    data: T,
}

enum Commands {
    Ping,
    Echo(String),
    Set(String, Value, Option<u64>),
    Get(String),
    Del(Vec<String>),
    Exists(String),
    Expire(String, Option<u64>),
    Ttl(String),
    Persist(String),
    Info,
    Error(String),
}

impl Commands {
    fn dispatcher(&self) {
        // these call the core functions based on the command
        match self {
            Commands::Echo(message) => println!(),
            Commands::Ping => println!(),
            Commands::Set(key, value, timeout) => println!(),
            Commands::Get(key) => println!(),
            Commands::Del(key) => println!(),
            Commands::Exists(key) => println!(),
            Commands::Expire(key, timeout) => println!(),
            Commands::Ttl(key) => println!(),
            Commands::Persist(key) => println!(),
            Commands::Info => println!(),
            Commands::Error(code) => println!(),
        }
    }
}

struct CommandsQueue {
    queue: Vec<RespType>,
    commands_dropped: u64,
}

fn handle_connections(stream: TcpStream) -> std::io::Result<()> {
    let perr_addr: std::net::SocketAddr = stream.peer_addr()?;
    println!("Client {:?} connected", perr_addr);

    let buf_reader: BufReader<TcpStream> = BufReader::new(stream);

    let mut command_queue: CommandsQueue = CommandsQueue {
        commands_dropped: 0,
        queue: Vec::new(),
    };

    for line in buf_reader.lines() {
        let line: String = line?;
        // parse it out into a commands
        // ok so things to think about for edge cases
        // array would need to keep grabbing multiple loops
        // other types take two loops to grab their data
        // if we hit another start byte and we are not working with an array
        // this would be a new command we hit now, so they were back to back
        let byte_stream: &[u8] = line.as_bytes();
        let first_byte: u8 = byte_stream[0];
        let data_type: Option<RespType> = find_resp_type(first_byte);
        if data_type.is_none() {
            // send back an error code for not valid type
            break;
        }
    }

    Ok(())
}

fn find_resp_type(first_byte: u8) -> Option<RespType> {
    match first_byte {
        47 => Some(RespType::SimpleStrings(None)),
        45 => Some(RespType::SimpleErrors(None)),
        58 => Some(RespType::Integers(None)),
        36 => Some(RespType::BulkStrings(None)),
        42 => Some(RespType::Arrays(None)),
        95 => Some(RespType::Nulls(None)),
        35 => Some(RespType::Booleans(None)),
        _ => None,
    }
}

fn parse_resp(bytes: &[u8], resp_type: RespType) -> Vec<&str> {
    match resp_type {
        _ => vec![""],
    }
}

fn main() {
    let engine: Arc<RwLock<Engine>> = Arc::new(RwLock::new(Engine::default())); // shared in threads for diapatcher to use
}
