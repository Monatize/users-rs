use web3::signing::recover;

use crate::utils::eth_message::eth_message;

use std::io::{Error, ErrorKind};
#[derive(Debug)]
struct SigError(String);

/// Returns the public key address from a signature, ex. ``0x704C47D6FF3E21216A2087c359010CB2140cA047``
/// 
/// # Arguments
/// 
/// * `address` - The Ethereum Address used to sign the signature | Type: ``String`` 
/// 
/// * `nonce` - The generated 8 character nonce | Type: ``String``
/// 
/// * `signature` - The signature hash | Type: ``String``
/// 
/// # Examples
/// 
/// ```
/// use crate::utils::get_pubkey::get_pubkey;
/// let pubkey: Result<String, Error> = get_pubkey(address, nonce, signature);
/// match pubkey {
///     Ok(pubkey: String) => {
///         println!("{}", pubkey);
///     }
///     Err(err: std::io::Error) => {
///         println!("{:?}", err);
///     }
/// };
/// ```
pub fn get_pubkey(address: String, nonce: String, signature: String) -> Result<String, Error> {
    let message = eth_message(&address, &nonce); // * Get the ETH signed message in byte form
    let signature = &signature[2..]; // * Remove 0x from the signature
    let signature = hex::decode(signature); // * Convert the String to a Vec<u8> for recover()
    match signature {
        Ok(signature) => {
            let pubkey = recover(&message, &signature[..64], 0);
            match pubkey {
                Ok(pubkey) => {
                    Ok(format!("{:02X?}", pubkey))
                }
                Err(e) => {
                    Err(Error::new(ErrorKind::InvalidData, format!("Recovery error: {}", e)))
                }
            }
        }
        Err(e) => {
            Err(Error::new(ErrorKind::InvalidData, format!("Hex Decode Error: {}", e)))
        }
    }
}