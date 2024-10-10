use dopechain_rust_lib::sdk::{destroy_contract, get_block_id, get_sender_id, log, read_storage, transfer_currency, write_storage};

#[no_mangle]
pub extern "C" fn _run() {
    write_storage("balance", "someverylonguser");
    let value = read_storage("balance").unwrap();
    transfer_currency(&value, 150);
    let sender = get_sender_id();
    let block_number = get_block_id();
    write_storage("sender", &sender);
    transfer_currency(&sender, block_number);
    destroy_contract(&sender);
    log("The subroutine has exited successfully");
}
