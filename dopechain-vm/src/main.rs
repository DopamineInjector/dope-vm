use std::{env::args, process::exit};

use dopechain_vm::run_binary;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Error while trying to run dopechain-vm: invalid argument count");
        exit(1);
    }
    let binary_path = args.get(1).unwrap();
    let res = run_binary(binary_path);
    if let Err(err) = res {
        println!("Error while running dopechain-vm: {err}");
    }
}
