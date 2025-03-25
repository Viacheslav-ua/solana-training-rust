use solana_sdk::signature::{Keypair, Signer};


fn main() {
    let keypair: Keypair = Keypair::new();
    
    let private_key: [u8; 64] = keypair.to_bytes();
    let public_key: solana_sdk::pubkey::Pubkey = keypair.pubkey();

    println!("Public Key: {}", public_key);
    println!("Private Key: {:?}", private_key);
}
