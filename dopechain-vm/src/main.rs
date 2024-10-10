use std::path::PathBuf;
use clap::Parser;
use dopechain_vm::run_binary;

#[derive(Parser)]
struct CliArgs {
    binary: PathBuf,
    entrypoint: String,
    blockaddr: String,
    db: PathBuf,
    sender: String,
    block_number: u64,
}

fn main() {
    let args = CliArgs::parse();
    let res = run_binary(
        args.binary, 
        args.entrypoint,
        args.blockaddr, 
        args.db,
        args.sender,
        args.block_number);
    if let Err(err) = res {
        println!("Error while running dopechain-vm: {err}");
    }
}
