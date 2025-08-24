use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    expire_on: Option<u128>,
}

impl Record {
    fn timeout_check(&self) -> bool {
        let epoch_mili: Option<u128> = get_epoch_time();
        if self.expire_on > epoch_mili || self.expire_on == None {
            return true;
        }
        false
    }

    fn set_expire_timer(&mut self, epoch_time: u128) {
        self.expire_on = Some(epoch_time);
    }

    fn check_time_left(&self) -> i64 {
        let current_epoch_time: Option<u128> = get_epoch_time();
        if self.expire_on.is_none() {
            return -1;
        }
        if current_epoch_time >= self.expire_on {
            -2
        } else {
            let time_left: u128 = self.expire_on.unwrap() - current_epoch_time.unwrap();
            (time_left / 1000).try_into().unwrap_or(i64::MAX)
        }
    }
}

struct Engine {
    key_store: HashMap<String, Record>,
    counters: Counters,
    start_time: SystemTime,
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
        expire_on: get_epoch_time(),
    }
}

fn get_epoch_time() -> Option<u128> {
    let now: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = now.duration_since(UNIX_EPOCH).unwrap();
    Some(duration_since_epoch.as_millis())
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
