contract;

storage {
    /// counter
    counter: u64 = 0,
}
 
abi Counter {
    #[storage(read, write)]
    fn set(value: u64);
 
    #[storage(read)]
    fn get() -> u64;

    fn get_contract_bytecode(contract_id: ContractId) -> u64;

    fn get_first_8_bytes(contract_id: ContractId) -> [u8; 8];

    fn get_bytecode(contract_id: ContractId) -> Vec<u8>;
}
 
impl Counter for Contract {
    #[storage(read, write)]
    fn set(value: u64) {
        let incremented = storage.counter.read() + value;
        storage.counter.write(incremented);
    }
 
    #[storage(read)]
    fn get() -> u64  {
        storage.counter.read()
    }

    fn get_contract_bytecode(contract_id: ContractId) -> u64 {
        _get_code_size(contract_id)
    }

    fn get_first_8_bytes(contract_id: ContractId) -> [u8; 8] {
        let mut bytecode_array: [u8; 8] = [0; 8];
        let mut counter = 0;
       while counter < 2 {
            bytecode_array[counter] = _get_bytecode_byte_at(contract_id, counter);
            counter += 1;
        }
        bytecode_array
    }

    fn get_bytecode(contract_id: ContractId) -> Vec<u8> {
        _get_bytecode(contract_id)
    }

}

fn _get_bytecode(contract_id: ContractId) -> Vec<u8> {
    let mut bytecode: Vec<u8> = Vec::new();
    //let bytecode_size = _get_code_size(contract_id);
    let mut counter = 0;
    while counter < 10 {
        bytecode.push(_get_bytecode_byte_at(contract_id, counter));
        counter += 1;
    }

    bytecode
}


fn _get_code_size(contract_id: ContractId) -> u64 {
    asm(target: contract_id, length) {
        csiz length target;
        length: u64
    }
}

fn _get_bytecode_byte_at(contract_id: ContractId, pos: u64) -> u8 {
    let mut bytecode: u8 = 0;
    let amount = 1;
    asm(target: contract_id, bt: &bytecode, pos: pos, amount: amount) {
        ccp bt target pos amount;
    }
    bytecode
}