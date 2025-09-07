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

enum FirstByte {
    SimpleStrings(u8),
    SimepleErrors(u8),
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

fn handle_connections(stream: TcpStream) -> std::io::Result<()> {
    let perr_addr: std::net::SocketAddr = stream.peer_addr()?;
    println!("Client {:?} connected", perr_addr);

    let buf_reader: BufReader<TcpStream> = BufReader::new(stream);

    for line in buf_reader.lines() {
        let line: String = line?;
        // parse it out into a commands
    }

    Ok(())
}

// fn recv_bytes() {}

fn parse_resp(bytes: Vec<u8>) -> String {
    "string".to_string()
}

fn main() {
    let engine: Arc<RwLock<Engine>> = Arc::new(RwLock::new(Engine::default())); // shared in threads for diapatcher to use
}
