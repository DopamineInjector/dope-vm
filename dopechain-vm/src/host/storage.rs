use std::path::PathBuf;

use rocksdb::DB;

pub fn insert(db_path: &PathBuf, key: &str, value: &str) {
    let db = DB::open_default(db_path).unwrap();
    db.put(key, value).unwrap();
}

pub fn get(db_path: &PathBuf, key: &str) -> Option<String> {
    let db = DB::open_default(db_path).unwrap();
    match db.get(key).unwrap() {
        Some(val) => Some(String::from_utf8(val).unwrap()),
        None => None
    }
}
