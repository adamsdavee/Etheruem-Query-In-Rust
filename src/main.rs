#[macro_use]
extern crate serde_derive;

mod Blockchain_address;
mod Blockchain_status;
mod Blockchain_transaction;
mod Blockchain_info;

use {
    crate::Blockchain_status::BlockchainStatus,
    crate::Blockchain_address::BlockchainAddress,
    crate::Blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
};

fn blockchain_info_app(address: &str) {

    let blockchain_status: BlockchainStatus = Blockchain_info::blockchain_status_request();
    println!("\nQuerying {} - Chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);

    let blockchain_address: BlockchainAddress = Blockchain_info::blockchain_address_request(address);
    println!("Analyzing transaction for Bitcoin address {}", &blockchain_address.address);

    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time); // This is used to make the system sleep

    println!("You have a total of {} transactions", &blockchain_address.txs);

    println!("\n Do you want to query these transactions? (y/n)\n");

    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("\nWe will look up the following transactions \n");
        println!("{:?}", &blockchain_address.txids);
        thread::sleep(sleep_time);

        let mut balance: i32 = 0;
        for tx_ids in blockchain_address.txids {

            let mut subtotal_vin: i32 = 0;
            let mut subtotal_vout: i32 = 0;
            //

            let blockchain_transaction: BlockchainTransaction = Blockchain_info::blockchain_transaction_request(&tx_ids);

            let match_address = String::from(address);

            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_address) {
                    subtotal_vin += tx.value.parse::<i32>().unwrap();
                }
            }

            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    subtotal_vout += tx.value.parse::<i32>().unwrap();
                }
            }


            balance  =  subtotal_vin - subtotal_vout;

            println!("-----------------------------------------------------");
            println!("TX ID:          {}", &blockchain_transaction.txid);
            println!("IN:    {}", &subtotal_vout);
            println!("OUT:   {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("-----------------------------------------------------");

        };

        println!("CURRENT BALANCE:     {}", &balance);
        // println!("         IN BTC:     {}\n\n", balance as f32 * 0.00000001);
    };
}

fn main() {
    
    let entered_address = dotenv::var("WALLET").expect("Error reading env var");
    blockchain_info_app(&entered_address);
    println!("Hello, world!");
}
