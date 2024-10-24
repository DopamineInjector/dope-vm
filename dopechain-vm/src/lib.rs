#![feature(once_cell_get_mut)]
use std::{error::Error, fs, path::PathBuf};

use linker::create_linker;
use wasmtime::{AsContext, Engine, Memory, MemoryType, Module, Store};

mod host;
mod linker;

pub const MEMORY_INITIAL_PAGES: usize = 5000;

pub fn run_binary(
    binary_path: PathBuf, 
    entrypoint: String,
    blockchain_id: String, 
    db_path: String,
    sender_id: String,
    block_number: u64,
    function_args: String
) -> Result<String, Box<dyn Error>> {
    let binary_code = fs::read(binary_path)?;
    let engine = Engine::default();
    let module = Module::new(&engine, binary_code)?;
    let mut store = Store::new(&engine, String::new());
    let memory = Memory::new(&mut store, MemoryType::new(1000, None))?;
    let mut linker = create_linker(blockchain_id, sender_id, block_number, db_path, function_args, memory, &engine);
    linker.define(&mut store, "env", "memory", memory.clone())?;
    let instance = linker.instantiate(&mut store, &module)?;
    // Run wasm binary
    // Look for return-less function first
    match instance.get_typed_func::<(), ()>(&mut store, &entrypoint) {
        Ok(func) => {
            let _ = func.call(&mut store, ());
            Ok(String::new())
        }
        Err(_) => {
            // Called function returns a string value
            let main_func = instance.get_typed_func::<(), i64>(&mut store, &entrypoint)?;
            main_func.call(&mut store, ())?;
            Ok(store.data().to_string())
        }
    }
}

