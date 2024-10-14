# dope-vm
A virtual machine used for execution of smart contracts on the blockchain

## How it works
todo, i am too tired to write documenatation

## Running the vm
The vm should be compiled as a binary file. It accepts the following flags:
1. -b / --binary => path to compiled wasm binary to run
2. --blockaddr => blockchain address of the SmartContract which is being executed
3. -d / --db => path to RocksDB directory used for on-chain storage
4. -s / --sender => wallet adress of the transaction sender
5. --block-number => current blockchain block number
6. -e / --entrypoint => name of the function to be executed 
7. -a / --args => arguments to the function to be executed, in json format

## DLC: test dopechain vm calls
1.  
./dopechain-vm -b ../wasm32-unknown-unknown/release/dopechain_contracts.wasm --blockaddr someaddr -d test.db -s sender_id --block-number 2137 -e _mint -a '{"owner": "owner1", "metadata_uri": "http://google.com"}'
2. 
./dopechain-vm -b ../wasm32-unknown-unknown/release/dopechain_contracts.wasm --blockaddr someaddr -d test.db -s sender_id --block-number 2137 -e _owner_of -a '0'
3.
./dopechain-vm -b ../wasm32-unknown-unknown/release/dopechain_contracts.wasm --blockaddr someaddr -d test.db -s sender_id --block-number 2137 -e _owned_by -a '"owner2"'
4.
./dopechain-vm -b ../wasm32-unknown-unknown/release/dopechain_contracts.wasm --blockaddr someaddr -d test.db -s sender_id --block-number 2137 -e _transfer_from -a '{"from": "owner1", "to": "owner2", "token_id": 5}'
5.
./dopechain-vm -b ../wasm32-unknown-unknown/release/dopechain_contracts.wasm --blockaddr someaddr -d test.db -s sender_id --block-number 2137 -e _get_metadata -a 5
