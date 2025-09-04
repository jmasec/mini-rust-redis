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

use kvd_core::Value;

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

// fn handle_connections() {}

// fn recv_bytes() {}

fn parse_resp(bytes: Vec<u8>) -> String {
    "string".to_string()
}
