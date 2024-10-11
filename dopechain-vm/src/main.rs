use std::path::PathBuf;
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
    db: PathBuf,
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
    if let Err(err) = res {
        println!("Error while running dopechain-vm: {err}");
    }
}
