use solana_sdk::signature::{Keypair, Signer};
use std::fs;

fn main() {
    let keypair: Keypair = Keypair::new();
    
    let private_key: String = bs58::encode(keypair.to_bytes()).into_string();
    let public_key: String = keypair.pubkey().to_string();

    println!("Public Key: {}", public_key);
    println!("Private Key: {}", private_key);

    let env_content: String = format!("PRIVATE_KEY={}\nPUBLIC_KEY={}\n", private_key, public_key);
    fs::write("./.env", env_content).expect("Failed to write to .env file");

    println!("✅ The keys saved to .env file");
}
