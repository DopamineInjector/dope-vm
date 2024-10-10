#![feature(once_cell_get_mut)]
use std::{error::Error, fs, path::PathBuf};

use linker::create_linker;
use wasmtime::{Engine, Memory, MemoryType, Module, Store};

mod host;
mod linker;

pub const MEMORY_INITIAL_PAGES: usize = 1000;

pub fn run_binary(
    binary_path: PathBuf, 
    entrypoint: String,
    blockchain_id: String, 
    db_path: PathBuf,
    sender_id: String,
    block_number: u64
) -> Result<(), Box<dyn Error>> {
    let binary_code = fs::read(binary_path)?;
    let engine = Engine::default();
    let module = Module::new(&engine, binary_code)?;
    let mut store = Store::new(&engine, ());
    let memory = Memory::new(&mut store, MemoryType::new(1000, None))?;
    let mut linker = create_linker(blockchain_id, sender_id, block_number, db_path, memory, &engine);
    linker.define(&mut store, "env", "memory", memory.clone())?;
    let instance = linker.instantiate(&mut store, &module)?;
    // Run wasm binary
    let main_func = instance.get_typed_func::<(), ()>(&mut store, &entrypoint)?;
    main_func.call(&mut store, ())?;
    Ok(())
}

