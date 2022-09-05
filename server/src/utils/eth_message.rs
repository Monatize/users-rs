use web3::signing::{keccak256};

pub fn eth_message(address: &String, nonce: &String) -> [u8; 32] {
    let message = format!("Thanks for using Monatize. To authenticate, please sign this message.\nAddress: {}\nNonce: {}\n\nTOS: https://monatize.it/legal/tos", address, nonce);

    println!("{}", message);
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        ).as_bytes()
    )
}