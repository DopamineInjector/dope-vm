use std::ffi::{CStr, CString};

#[link(wasm_import_module = "env")]
extern "C" {
    fn storage_read(key_ptr: i32, key_len: i32) -> i32;
    fn storage_write(key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32);
    fn transfer(recipient_ptr: i32, recipient_len: i32, amount: u64);
    fn get_sender() -> i32;
    fn get_block_number() -> u64;
    fn log_message(message_ptr: i32, message_len: i32);
    fn self_destruct(recipient_ptr: i32, recipient_len: i32);
    fn env_get_args() -> i32;
    fn env_return_value(ptr: i32, len: i32);
}

pub fn read_storage(key: &str) -> Option<String> {
    let ckey = CString::new(key).unwrap();
    let len = ckey.count_bytes();
    let ptr = ckey.as_ptr();
    let mut value: Option<String> = None;
    unsafe {
        let offset = storage_read(ptr as i32, len as i32);
        if offset != 0 {
            let res = CStr::from_ptr(offset as *const i8);
            let stringified = res.to_str().unwrap().to_string();
            value = Some(stringified)
        }
    }
    return value
}

pub fn write_storage(key: &str, value: &str) {
    let ckey = CString::new(key).unwrap();
    let key_len = ckey.count_bytes() as i32;
    let key_ptr = ckey.as_ptr() as i32;
    let cval = CString::new(value).unwrap();
    let val_len = cval.count_bytes() as i32;
    let val_ptr = cval.as_ptr() as i32;
    unsafe {
        storage_write(key_ptr, key_len, val_ptr, val_len);
    }
}

pub fn transfer_currency(recipient: &str, amount: u64) {
    let filtered_input: String = recipient.chars().filter(|&c| c != '\0').collect();
    let crec = CString::new(filtered_input).unwrap();
    let len = crec.count_bytes() as i32;
    let ptr = crec.as_ptr() as i32;
    unsafe {
        transfer(ptr, len, amount);
    }
}

pub fn get_sender_id() -> String {
    let mut val = String::new();
    unsafe {
        let offset = get_sender();
        let res = CStr::from_ptr(offset as *const i8);
        let stringified = res.to_str().unwrap().to_string();
        val = stringified;
    }
    val
}

pub fn get_block_id() -> u64 {
    let mut res: u64 = 0;
    unsafe {
        let block_number = get_block_number();
        res = block_number;
    }
    res
}

pub fn log(message: &str) {
    let cmes = CString::new(message).unwrap();
    let len = cmes.count_bytes() as i32;
    let ptr = cmes.as_ptr() as i32;
    unsafe {
        log_message(ptr, len);
    }
}

pub fn destroy_contract(recipient: &str) {
    let cmes = CString::new(recipient).unwrap();
    let len = cmes.count_bytes() as i32;
    let ptr = cmes.as_ptr() as i32;
    unsafe {
        self_destruct(ptr, len);
    }
}

pub fn get_user_args() -> String {
    let mut val = String::new();
    unsafe {
        let offset = env_get_args();
        let res = CStr::from_ptr(offset as *const i8);
        let stringified = res.to_str().unwrap().to_string();
        val = stringified;
    }
    val
}

pub fn write_return(return_val: &str) {
    let alloced = CString::new(return_val).unwrap();
    let len = alloced.count_bytes() as i32;
    let ptr = alloced.as_ptr() as i32;
    unsafe {
        env_return_value(ptr, len);
    }
}
