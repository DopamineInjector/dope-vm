pub fn transfer(recipient: &str, amount: u64) {
    eprintln!("[TODO] Transferring {amount} to {recipient}");
}

pub fn log(message: &str) {
    eprintln!("[LOG]: {message}");
}

pub fn destroy(contract: &str, recipient: &str) {
    eprintln!("[TODO] destroying {contract}, sending remaining balance to {recipient}");
}
