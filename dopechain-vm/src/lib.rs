use std::{error::Error, fs};

use wasmtime::{Engine, Instance, Module, Store};

pub const MAIN_FUNCTION_NAME: &'static str = "_run";

pub fn run_binary(binary_path: &str) -> Result<(), Box<dyn Error>> {
    let binary_code = fs::read(binary_path)?;
    let engine = Engine::default();
    let module = Module::new(&engine, binary_code)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let main_func = instance.get_typed_func::<(), ()>(&mut store, MAIN_FUNCTION_NAME)?;
    main_func.call(&mut store, ())?;
    Ok(())
}

