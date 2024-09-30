use std::str::FromStr;

use fuels::{
    prelude::*, 
    types::ContractId, 
    crypto::SecretKey,
    programs::responses::*,
};

use rand::Rng;

use std::{fs};

const CONTRACT_BYTECODE_PATH: &str =
    "./out/debug/fuel-counter.bin";

// Load abi from json
abigen!(Contract(
    name = "Counter",
    abi = "out/debug/fuel-counter-abi.json"
));

async fn get_contract_instance() -> (Counter<WalletUnlocked>, ContractId) {
    // Launch a local network and deploy the contract
    //let provider = Provider::connect("127.0.0.1:4000").await.unwrap();
    let provider = Provider::connect("testnet.fuel.network").await.unwrap();

    let secret = match SecretKey::from_str(
        "<ADD_YOUR_KEY_HERE>"
    ) {
        Ok(value) => value,
        Err(e) => panic!("unable to create secret: {}", e),
    };

    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    // Generate a random 32-byte array
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);

    let salt = Salt::new(bytes);

    //let gas_limit = provider.consensus_parameters().max_gas_per_tx() * 0.8;

    let id = Contract::load_from(
        "./out/debug/fuel-counter.bin",
        LoadConfiguration::default().with_salt(salt),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default().with_script_gas_limit(400000))
    .await
    .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    (instance, id.into())
}

mod success {

    use super::*;

    #[tokio::test]
    async fn test_deploy() {
        let (_instance, _id) = get_contract_instance().await;
    
        let result = _instance.methods().get().call().await.unwrap();
        display_results(result);
    }

    #[tokio::test]
    async fn test_code_size() {
        let (_instance1, id1) = get_contract_instance().await;
        let (instance2, _id2) = get_contract_instance().await;
        let bech32_id1 = Bech32ContractId::from(id1);

        let result = instance2
            .methods()
            .get_contract_bytecode(id1)
            .with_tx_policies(
                TxPolicies::default()
                .with_script_gas_limit(400000)
            )
            .with_contract_ids(&[bech32_id1])
            .call()
            .await
            .unwrap();

        println!("Bytecode size: {}", result.value);
        println!("Gas used: {}", result.gas_used);
    }

    #[tokio::test]
    async fn test_bytecode() {
        let (_instance1, id1) = get_contract_instance().await;
        let (instance2, _id2) = get_contract_instance().await;
        let bech32_id1 = Bech32ContractId::from(id1);

        let result = instance2
            .methods()
            .get_first_8_bytes(id1)
            .with_tx_policies(
                TxPolicies::default()
                .with_script_gas_limit(400000)
            )
            .with_contract_ids(&[bech32_id1])
            .call()
            .await
            .unwrap();

        println!("Bytecode: {:?}", result.value);

        //println!("Bytecode: {:?}", simple_contract_bytecode());
        //display_results(result);
        println!("Gas used: {}", result.gas_used);
        
    }

}

fn simple_contract_bytecode() -> Vec<u8> {
    fs::read(CONTRACT_BYTECODE_PATH).unwrap()
}

fn display_results<T: std::fmt::Display>(result: CallResponse<T>) {
    println!("value: {}", result.value);
    println!("Gas used: {}", result.gas_used);
    println!("TX ID: {:?}", result.tx_id);
    let receipts_count = result.receipts.len();

    println!("------ Receipts ------");
    println!("Number of receipts: {}", receipts_count);
    for receipt in result.receipts {
        match receipt {
            Receipt::Call {
                id,
                to,
                amount,
                asset_id,
                gas,
                param1,
                param2,
                pc,
                is,
            } => {
                println!("Call receipt:");
                println!("   id: {:?}", id);
                println!("   to: {:?}", to);
                println!("   amount: {:?}", amount);
                println!("   asset_id: {:?}", asset_id);
                println!("   gas: {:?}", gas);
                println!("   param1: {:?}", param1);
                println!("   param2: {:?}", param2);
                println!("   pc: {:?}", pc);
                println!("   is: {:?}", is);
            }
            Receipt::Mint {
                sub_id,
                contract_id,
                val,
                pc,
                is,
            } => {
                println!("Mint Receipt:");
                println!("  sub_id: {:?}", sub_id);
                println!("  contract_id: {:?}", contract_id);
                println!("  val: {}", val);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::Burn {
                sub_id,
                contract_id,
                val,
                pc,
                is,
            } => {
                println!("Burn Receipt:");
                println!("  sub_id: {:?}", sub_id);
                println!("  contract_id: {:?}", contract_id);
                println!("  val: {}", val);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::Return { id, val, pc, is } => {
                println!("Return Receipt:");
                println!("  id: {:?}", id);
                println!("  val: {}", val);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::ReturnData {
                id,
                ptr,
                len,
                digest,
                pc,
                is,
                data,
            } => {
                println!("ReturnData Receipt:");
                println!("  id: {:?}", id);
                println!("  ptr: {}", ptr);
                println!("  len: {}", len);
                println!("  digest: {:?}", digest);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
                println!("  data: {:?}", data);
            }
            Receipt::Panic {
                id,
                reason,
                pc,
                is,
                contract_id,
            } => {
                println!("Panic Receipt:");
                println!("  id: {:?}", id);
                println!("  reason: {:?}", reason);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
                println!("  contract_id: {:?}", contract_id);
            }
            Receipt::Revert { id, ra, pc, is } => {
                println!("Revert Receipt:");
                println!("  id: {:?}", id);
                println!("  ra: {}", ra);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::Log {
                id,
                ra,
                rb,
                rc,
                rd,
                pc,
                is,
            } => {
                println!("Log Receipt:");
                println!("  id: {:?}", id);
                println!("  ra: {}", ra);
                println!("  rb: {}", rb);
                println!("  rc: {}", rc);
                println!("  rd: {}", rd);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::LogData {
                id,
                ra,
                rb,
                ptr,
                len,
                digest,
                pc,
                is,
                data,
            } => {
                println!("LogData Receipt:");
                println!("  id: {:?}", id);
                println!("  ra: {}", ra);
                println!("  rb: {}", rb);
                println!("  ptr: {}", ptr);
                println!("  len: {}", len);
                println!("  digest: {:?}", digest);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
                println!("  data: {:?}", data);
            }
            Receipt::Transfer {
                id,
                to,
                amount,
                asset_id,
                pc,
                is,
            } => {
                println!("Transfer Receipt:");
                println!("  id: {:?}", id);
                println!("  to: {:?}", to);
                println!("  amount: {}", amount);
                println!("  asset_id: {:?}", asset_id);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::TransferOut {
                id,
                to,
                amount,
                asset_id,
                pc,
                is,
            } => {
                println!("TransferOut Receipt:");
                println!("  id: {:?}", id);
                println!("  to: {:?}", to);
                println!("  amount: {}", amount);
                println!("  asset_id: {:?}", asset_id);
                println!("  pc: {}", pc);
                println!("  is: {}", is);
            }
            Receipt::ScriptResult { result, gas_used } => {
                println!("ScriptResult Receipt:");
                println!("  result: {:?}", result);
                println!("  gas_used: {}", gas_used);
            }
            Receipt::MessageOut {
                sender,
                recipient,
                amount,
                nonce,
                len,
                digest,
                data,
            } => {
                println!("MessageOut Receipt:");
                println!("  sender: {:?}", sender);
                println!("  recipient: {:?}", recipient);
                println!("  amount: {}", amount);
                println!("  nonce: {:?}", nonce);
                println!("  len: {}", len);
                println!("  digest: {:?}", digest);
                println!("  data: {:?}", data);
            }
        }
    }
    
}

/*#[tokio::test]
async fn test_fuzz_inc_dec() {
    let (instance, _id) = get_contract_instance().await;
    let mut counter = 0;

    loop {
        if counter >= 255 {
            break;
        }
        let res_inc = instance.methods().inc().call().await.unwrap();
        //println!("Response: {:#?}", res_inc);
        assert_eq!(res_inc.value, 1);

        let res_get = instance.methods().get().call().await.unwrap();
        //println!("Response: {:#?}", res_inc);
        assert_eq!(res_get.value, 1);

        let res_dec = instance.methods().dec().call().await.unwrap();
        assert_eq!(res_dec.value, 0);
        counter += 1;
    }
}

#[tokio::test]
#[should_panic]
async fn test_dec_failure() {
    let (instance, _id) = get_contract_instance().await;
    instance.methods().dec().call().await.unwrap();
}*/