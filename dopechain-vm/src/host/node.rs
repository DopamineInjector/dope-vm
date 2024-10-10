pub fn transfer(recipient: &str, amount: u64) {
    println!("[TODO] Transferring {amount} to {recipient}");
}

pub fn log(message: &str) {
    println!("[LOG]: {message}");
}

pub fn destroy(contract: &str, recipient: &str) {
    println!("[TODO] destroying {contract}, sending remaining balance to {recipient}");
}
