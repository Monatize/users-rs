use web3::signing::{keccak256};

/// Returns a signed ETH message in bytes
/// 
/// # Arguments
/// 
/// * `address` - The Ethereum Address used to sign the signature | Type: ``String`` 
/// 
/// * `nonce` - The generated 8 character nonce | Type: ``String``
/// 
/// # Examples
/// 
/// ```
/// use crate::utils::eth_message::eth_message;
/// let eth_message: [u8; 32] = eth_message(address, nonce);
/// println!("{:?}", eth_message);
/// ```
pub fn eth_message(address: &String, nonce: &String) -> [u8; 32] {
    let message = format!("Thanks for using Monatize. To authenticate, please sign this message.\nAddress: {}\nNonce: {}\n\nTOS: https://monatize.it/legal/tos", address, nonce);

    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        ).as_bytes()
    )
}