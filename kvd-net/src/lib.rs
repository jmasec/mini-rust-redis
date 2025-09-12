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

// RespTypes of the data that is being parsed, Pieces of each of the data
enum RespType {
    SimpleStrings(Option<HangingData<String>>),
    SimpleErrors(Option<HangingData<String>>),
    Integers(Option<HangingData<u64>>),
    BulkStrings(Option<u64>, Option<HangingData<String>>),
    Arrays(Option<u64>),
    Nulls(Option<HangingData<char>>),
    Booleans(Option<HangingData<char>>),
}

// Peices of the data that are waiting to be formed into a command, hold raw data
struct HangingData<T> {
    data: T,
}

// This stores the data of one whole command until there is the end of message
// we will handle arrays first since that seems to be the common way to send
// a command, array of bulk strings
struct CommandsDataQueue {
    queue: Vec<RespType>,
    commands_dropped: u64,
    done: bool,
}

impl CommandsDataQueue {
    fn add_data_to_queue(&mut self, resp: RespType) {
        self.queue.push(resp);
    }
}

// these are the literal string commands, ECHO == Echo, need a match case for this still, if string GET then Get(key)
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
        // these call the core functions based on the command, actually we will probably need to call functions in net
        // that call core functions since we need to package a response up as well
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

fn handle_connections(stream: TcpStream) -> std::io::Result<()> {
    let perr_addr: std::net::SocketAddr = stream.peer_addr()?;
    println!("Client {:?} connected", perr_addr);

    let buf_reader: BufReader<TcpStream> = BufReader::new(stream);

    let mut command_data_queue: CommandsDataQueue = CommandsDataQueue {
        commands_dropped: 0,
        queue: Vec::new(),
        done: false,
    };

    for line in buf_reader.lines() {
        let line: String = line?;
        // parse it out into a commands
        // ok so things to think about for edge cases
        // array would need to keep grabbing multiple loops
        // other types take two loops to grab their data
        // if we hit another start byte and we are not working with an array
        // this would be a new command we hit now, so they were back to back
        // each data we parse out, which is really each line we are getting in the loop
        // and we create the RESP from first byte to start, then fill in the data from
        // each new line will have a data type byte except the actual data
        // int is data right after byte, array has size after byte, bulk string has size after byte
        // if I append every type to the command queue, then I could technically just grab the bulk strings data from there
        // or everything except the array data basically, or use array len, to loop through cmd queue or something
        let byte_stream: &[u8] = line.as_bytes();
        let first_byte: u8 = byte_stream[0];
        let data_type: Option<RespType> = find_resp_type(first_byte);

        if data_type.is_none() && (matches!(command_data_queue.queue[0], RespType::Arrays(_, _))) {
            // if no type, then we just need to read in a string, and append to the last thing on the queue (Bulk String)
            return Ok(());
        }

        // parse_resp(data_type)

        command_data_queue.add_data_to_queue(data_type.unwrap());

        if command_data_queue.done {
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
        36 => Some(RespType::BulkStrings(Some(0), None)),
        42 => Some(RespType::Arrays(Some(0))),
        95 => Some(RespType::Nulls(None)),
        35 => Some(RespType::Booleans(None)),
        _ => None,
    }
}

fn parse_resp(bytes: &[u8], resp_type: RespType) -> Option<Vec<&str>> {
    match resp_type {
        _ => None,
    }
}

fn main() {
    let engine: Arc<RwLock<Engine>> = Arc::new(RwLock::new(Engine::default())); // shared in threads for diapatcher to use
}
