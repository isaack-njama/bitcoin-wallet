extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc_json::AddressType;
use std::env;

fn main() {
    let rpc_user = env::var("RPC_USER").expect("RPC_USER not found in environment");
    let rpc_password = env::var("RPC_PASSWORD").expect("RPC_PASSWORD not found in environment");

    let rpc = Client::new("http://localhost:18332",
                        Auth::UserPass(rpc_user.to_string(),
                                        rpc_password.to_string())).unwrap();

    let wallet_name = "isaack_wallet";
    let disabled_private_keys = Some(false);
    let blank = Some(false);
    let passphrase = Some("isaack");
    let avoid_reuse = Some(true);

    let _wallet_identifier = rpc.create_wallet(wallet_name, disabled_private_keys, blank, passphrase, avoid_reuse);

    let _user_wallet = rpc.load_wallet("wallet_identifier");
    
    let wallet_info = rpc.get_wallet_info().unwrap();
    println!("Wallet name: {}", wallet_info.wallet_name);

    let new_address = rpc.get_new_address(Some("isaack_wallet_address"), Some(AddressType::Bech32));
    match new_address {
        Ok(wallet_address) => println!("Wallet Address: {:?}", wallet_address),
        Err(err) => println!("Error creating wallet address: {:?}", err),
    }
}
