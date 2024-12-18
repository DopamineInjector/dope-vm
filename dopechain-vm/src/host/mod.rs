use std::path::PathBuf;

use wasmtime::{AsContext, Memory, StoreContext, StoreContextMut };

use crate::host::utils::put_string_in_memory;

use self::utils::get_string_from_memory;

mod storage;
mod utils;
mod node;
mod mmu;

pub fn env_get_args(
    args: &str,
    memory: Memory, 
    store_context: StoreContextMut<String>,
) -> i32 {
    put_string_in_memory(args, memory, store_context)
}

pub fn env_return_value(
    ptr: usize, 
    len: usize, 
    memory: Memory, 
    mut store_context: StoreContextMut<String>,
) {
    let val = get_string_from_memory(ptr, len, memory, store_context.as_context());
    *store_context.data_mut() = val;
}

pub fn storage_write(
    key_ptr: usize, 
    key_len: usize, 
    value_ptr: usize, 
    value_len: usize, 
    memory: Memory, 
    store_context: StoreContext<String>,
    contract_prefix: &str, 
    db_url: &str
) {
    let key = get_string_from_memory(key_ptr, key_len, memory, store_context.as_context());
    let value = get_string_from_memory(value_ptr, value_len, memory, store_context.as_context());
    storage::insert(db_url, key.to_owned(), value.to_owned(), contract_prefix.to_owned());
}

pub fn storage_read(
    key_ptr: usize, 
    key_len: usize, 
    memory: Memory, 
    store_context: StoreContextMut<String>,
    contract_prefix: &str, 
    db_url: &str
) -> usize {
    let key = get_string_from_memory(key_ptr, key_len, memory, store_context.as_context());
    let res_offset = match storage::get(db_url, key.to_owned(), contract_prefix.to_owned()) {
        None => {
            0
        },
        Some(result) => {
            put_string_in_memory(&result, memory, store_context)
        }
    };
    res_offset as usize
}

pub fn transfer(
    recipient_ptr: usize, 
    recipient_len: usize, 
    amount: u64,
    memory: Memory, 
    store_context: StoreContext<String>,
) {
    let recipient = get_string_from_memory(recipient_ptr, recipient_len, memory, store_context);
    node::transfer(&recipient, amount);
}

pub fn get_sender(
    sender: &str,
    memory: Memory, 
    store_context: StoreContextMut<String>,
) -> i32 {
    put_string_in_memory(sender, memory, store_context)
}

pub fn get_block_number(
    block_number: u64,
) -> u64 {
    block_number
}

pub fn log_message(
    message_ptr: usize, 
    message_len: usize, 
    memory: Memory, 
    store_context: StoreContext<String>,
) {
    let message = get_string_from_memory(message_ptr, message_len, memory, store_context);
    node::log(&message);
}

pub fn self_destruct(
    recipient_ptr: usize, 
    recipient_len: usize, 
    contract_id: &str,
    memory: Memory, 
    store_context: StoreContext<String>,
) {
    let recipient = get_string_from_memory(recipient_ptr, recipient_len, memory, store_context);
    node::destroy(contract_id, &recipient);
}

pub fn initialize_storage(
    contract_prefix: &str, 
    db_url: &str
) {
    storage::initialize_storage(db_url, contract_prefix.to_string());
}
