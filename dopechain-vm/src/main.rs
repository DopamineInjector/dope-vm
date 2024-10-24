use std::{path::PathBuf, process::exit};
use clap::Parser;
use dopechain_vm::run_binary;

#[derive(Parser)]
struct CliArgs {
    #[arg(short, long)]
    binary: PathBuf,
    #[arg(short, long)]
    entrypoint: String,
    #[arg(long)]
    blockaddr: String,
    #[arg(short, long)]
    db: String,
    #[arg(short, long)]
    sender: String,
    #[arg(long)]
    block_number: u64,
    #[arg(short, long)]
    args: String,
}

fn main() {
    let args = CliArgs::parse();
    let res = run_binary(
        args.binary, 
        args.entrypoint,
        args.blockaddr, 
        args.db,
        args.sender,
        args.block_number,
        args.args);
    match res {
        Err(e) => {
            eprintln!("Error while running dopechain-vm: {e}");
            exit(1);
        },
        Ok(output) => {
            println!("{output}");
            exit(0);
        }
    }
    
}
