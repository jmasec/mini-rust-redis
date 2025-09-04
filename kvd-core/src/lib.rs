use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Implement the in-memory keyspace (HashMap)
// Store values + optional expiration timestamp
// Implement core commands: SET, GET, DEL, EXISTS
// Implement TTL commands: EXPIRE, PEXPIRE, TTL, PTTL, PERSIST
// Handle lazy expiration on reads and deletes
// Add simple counters (keys_total, expired_keys_total, etc.)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(Vec<u8>),
    // List(VecDeque<Vec<u8>>),
    // Set(HashSet<Vec<u8>>),
    // Hash(HashMap<String, Vec<u8>>),
    // SortedSet(BTreeMap<i64, Vec<u8>>),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    value: Value,
    expire_on: Option<u128>,
}

impl Record {
    fn check_timeout(&self) -> bool {
        let epoch_mili: Option<u128> = get_epoch_time();
        if self.expire_on > epoch_mili || self.expire_on.is_none() {
            return true;
        }
        false
    }

    fn set_expire(&mut self, epoch_time: u128) {
        self.expire_on = Some(epoch_time);
    }

    fn ttl(&self) -> i64 {
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

    fn remove_ttl(&mut self) {
        self.expire_on = None;
    }
}

pub struct Engine {
    key_store: HashMap<String, Record>,
    counters: Counters,
    start_time: SystemTime,
}

impl Engine {
    pub fn set_entry(&mut self, key: String, value: Value) {
        let record: Record = build_record(value);
        self.key_store.insert(key, record);
    }

    pub fn get_entry(&mut self, key: &str) -> Option<Value> {
        if let Some(record) = self.key_store.get(key) {
            if !record.check_timeout() {
                self.key_store.remove(key);
                return None;
            }
            Some(record.value.clone())
        } else {
            None
        }
    }

    // need to handle list of keys, take in a vector of keys
    pub fn del_entry(&mut self, key: &str) -> i32 {
        let record: Option<Record> = self.key_store.remove(key);
        if record.is_some() {
            return 1;
        }
        0
    }

    pub fn exists(&self, key: &str) -> bool {
        self.key_store.contains_key(key)
    }

    pub fn info(&self) -> (usize, usize) {
        (self.counters.keys_total, self.counters.expired_keys_total)
    }

    pub fn persist(&mut self, key: &str) {
        if self.exists(key) {
            self.key_store.get(key);
        }
    }

    pub fn start_time(&self) -> SystemTime {
        self.start_time
    }
}

fn build_record(value_to_add: Value) -> Record {
    Record {
        value: value_to_add,
        expire_on: None,
    }
}

fn get_epoch_time() -> Option<u128> {
    let now: SystemTime = SystemTime::now();
    let duration_since_epoch: Duration = now.duration_since(UNIX_EPOCH).unwrap();
    Some(duration_since_epoch.as_millis())
}

fn init_engine() -> Engine {
    Engine {
        key_store: HashMap::new(),
        counters: Counters {
            keys_total: 0,
            expired_keys_total: 0,
        },
        start_time: SystemTime::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_set_get() {
        let mut engine: Engine = init_engine();

        let key: String = "test".to_string();

        let expected: Value = Value::String(b"start".to_vec()); // this is our comparison value

        // Insert (set_entry takes ownership of Value)
        engine.set_entry(key.clone(), expected.clone());

        // Compare: get_entry returns Option<Value> (owned) in your code
        assert_eq!(engine.get_entry(&key), Some(expected));
    }
}
