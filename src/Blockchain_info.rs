use {
    dotenv,
    reqwest,
    tokio,
    serde_json::Result,
    crate::Blockchain_status::BlockchainStatus,
    crate::Blockchain_address::BlockchainAddress,
    crate::Blockchain_transaction::BlockchainTransaction,
};

const HOST_ROOT: &str = "https://eth-blockbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find key: API_KEY")) // expect handles error in case the variable is not there
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")
}


pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    // println!("{}", response);
    serde_json::from_str(&response).expect("Failed to parse JSON") // Converts JSON to a struct we can use
}

pub fn blockchain_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    println!("{}", response);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_transaction_request(transaction: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction].join(""));
    println!("{}", response);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}