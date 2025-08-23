use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

// Implement the in-memory keyspace (HashMap)
// Store values + optional expiration timestamp
// Implement core commands: SET, GET, DEL, EXISTS
// Implement TTL commands: EXPIRE, PEXPIRE, TTL, PTTL, PERSIST
// Handle lazy expiration on reads and deletes
// Add simple counters (keys_total, expired_keys_total, etc.)

// fn main() {
//     let start = Instant::now();

//     // Simulate work
//     thread::sleep(Duration::from_secs(2));

//     let elapsed = start.elapsed();
//     println!("Elapsed: {:.2?}", elapsed);
// }
#[derive(Clone)]
enum Value {
    String(Vec<u8>),
    List(VecDeque<Vec<u8>>),
    Set(HashSet<Vec<u8>>),
    Hash(HashMap<String, Vec<u8>>),
    SortedSet(BTreeMap<i64, Vec<u8>>),
}

struct Counters {
    keys_total: usize,
    expired_keys_total: usize,
}

impl Counters {
    fn increment_key_total(&mut self) {
        self.keys_total += 1;
    }

    fn increment_key_expires(&mut self) {
        self.expired_keys_total += 1;
    }
}

#[derive(Clone)]
struct Record {
    value: Value,
    expire_on: Instant, // current time in seconds + our timeout in seconds, then we get current time and check if its greater
}

impl Record {
    fn timeout_check(&self) -> bool {
        let now: Instant = Instant::now();
        if self.expire_on > now {
            return true;
        }
        false
    }

    fn set_expire_timer(&mut self, time: Instant) {
        self.expire_on = time;
    }

    fn check_time_left(&self) -> u64 {
        let now: Instant = Instant::now();
        let zero: Duration = Duration::ZERO;
        let elapsed_time: Duration = now - self.expire_on;
        if elapsed_time < zero {
            return zero.as_secs();
        }
        elapsed_time.as_secs()
        // need to make expire able to be None
    }
}

struct Engine {
    key_store: HashMap<String, Record>,
    counters: Counters,
    start_time: Instant,
}

impl Engine {
    fn add_new_record(&mut self, key: String, value: Value) {
        let record: Record = build_record(value);
        self.key_store.insert(key, record);
    }

    fn get_value(&self, key: &str) -> Option<Value> {
        let record: Option<Record> = self.key_store.get(key).cloned();
        if let Some(x) = record {
            if x.timeout_check() {
                return None;
            }
            Some(x.value)
        } else {
            None
        }
    }

    fn delete_key(&mut self, key: &str) -> bool {
        let record: Option<Record> = self.key_store.remove(key);
        record.is_some()
    }

    fn exists(&self, key: &str) -> bool {
        self.key_store.contains_key(key)
    }
}

fn build_record(value_to_add: Value) -> Record {
    Record {
        value: value_to_add,
        expire_on: Instant::now(),
    }
}

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
