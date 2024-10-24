use std::{path::PathBuf, sync::Arc};

use wasmtime::{AsContext, AsContextMut, Caller, Engine, Linker, Memory};

use crate::host::{env_get_args, env_return_value, get_block_number, get_sender, initialize_storage, log_message, self_destruct, storage_read, storage_write, transfer};

pub fn create_linker(
    contract_prefix: String, 
    sender_id: String, 
    block_number: u64, 
    db_url: String, 
    function_args: String, 
    memory: Memory, 
    engine: &Engine
) -> Linker<String> {
    let mut linker = Linker::new(engine);
    let contract_prefix = format!("data/contracts/{contract_prefix}");
    // Initialize storage
    initialize_storage(&contract_prefix, &db_url);
    let db_path = Arc::new(db_url);
    let contract_prefix = Arc::new(contract_prefix);
    // Get args string
    let _ = linker.func_wrap("env", "env_get_args", move |mut caller: Caller<'_, String>| {
        env_get_args(&function_args, memory, caller.as_context_mut())
    }); 
    let _ = linker.func_wrap("env", "env_return_value", move |mut caller: Caller<'_, String>, ptr: i32, len: i32| {
        env_return_value(ptr as usize, len as usize, memory, caller.as_context_mut());
    }); 
    // Storage read
    let dbp_read = db_path.clone();
    let cp_read = contract_prefix.clone();
    let _ = linker.func_wrap("env", "storage_read", move |mut caller: Caller<'_, String>, key_ptr: i32, key_len: i32 | {
        let offset = storage_read(key_ptr as usize, key_len as usize, memory, caller.as_context_mut(), &cp_read, &dbp_read);
        return offset as i32
    }); 
    //Storage write
    let dbp_write = db_path.clone();
    let cp_write = contract_prefix.clone();
    let _ = linker.func_wrap("env", "storage_write", move |caller: Caller<'_, String>, key_ptr: i32, key_len: i32, value_ptr: i32, value_len: i32| {
        storage_write(key_ptr as usize, key_len as usize, value_ptr as usize, value_len as usize, memory, caller.as_context(), &cp_write, &dbp_write);
    });
    // Transfer
    let _ = linker.func_wrap("env", "transfer", move |caller: Caller<'_, String>, recipient_ptr: i32, recipient_len: i32, amount: u64| {
        transfer(recipient_ptr as usize, recipient_len as usize, amount, memory, caller.as_context());
    });
    // Sender info
    let _ = linker.func_wrap("env", "get_sender", move |mut caller: Caller<'_, String>| {
        get_sender(&sender_id, memory, caller.as_context_mut())
    });
    // Block info
    let _ = linker.func_wrap("env", "get_block_number", move |caller: Caller<'_, String>| {
        get_block_number(block_number)
    });
    // Message log
    let _ = linker.func_wrap("env", "log_message", move |caller: Caller<'_, String>, message_ptr: i32, message_len: i32| {
        log_message(message_ptr as usize, message_len as usize, memory, caller.as_context());
    });
    // Destroy the contract
    let cp_destroy = Arc::clone(&contract_prefix);
    let _ = linker.func_wrap("env", "self_destruct", move |caller: Caller<'_, String>, recipient_ptr: i32, recipient_len: i32| {
        self_destruct(recipient_ptr as usize, recipient_len as usize, &cp_destroy, memory, caller.as_context())
    });
    
    linker
}
