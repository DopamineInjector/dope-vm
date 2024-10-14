use dopechain_rust_lib::{contracts::{Contract, Fetchable, OnChainVar}, sdk::{log, transfer_currency}};
use dopechain_rust_macros::contract_api;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct CustomContractArg {
    pub recipient: String,
    pub amount: u64
}

struct SampleContract {
    balance: OnChainVar<u64>,
    other_var: OnChainVar<String>
}

impl Contract for SampleContract {
    fn new() -> Self {
        SampleContract {
            balance: OnChainVar::new("balance"),
            other_var: OnChainVar::new("stuff"),
        }
    }
}

#[contract_api]
impl SampleContract {
    fn run(&mut self) {
        log("Balance from struct");
        match self.balance.get() {
            Some(bal) => log(&format!("{bal}")),
            None => log("No balance?")
        }
    }

    fn contract_test(&mut self, arg: CustomContractArg) -> String {
        let _ = self.other_var.get();
        log("Running other test function");
        transfer_currency(&arg.recipient, arg.amount);
        return "returned string".to_owned();
    }
}

