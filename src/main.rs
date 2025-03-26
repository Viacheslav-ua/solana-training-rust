use solana_sdk::signature::{Keypair, Signer};
use std::io;
use std::fs;

fn main() {
    loop {
        let mut action: String = String::new();

        println!("\x1b[32m----------------------------\x1b[0m");
        println!("Enter a command:");
        println!("\x1b[32m1\x1b[0m Generate and save keys");
        println!("\x1b[32m2\x1b[0m Read keys");
        println!("\x1b[32m0\x1b[0m Exit");

        match io::stdin().read_line(&mut action) {
            Ok(_) => {}
            Err(_e) => {
                println!("\x1b[31mInput error-{}\x1b[0m", _e)
            }
        }

        let command: u8 = action.trim().parse().unwrap();

        match &command {
            1 => { generate_keys(); },
            2 => println!("Read keys"),
            0 => { break; },
            _ => println!("\x1b[31mInvalid command\x1b[0m\nUse only \x1b[32m0\x1b[0m, \x1b[32m1\x1b[0m or \x1b[32m2\x1b[0m"),
        }
    }
    
}


fn generate_keys() {
    let keypair: Keypair = Keypair::new();

    let private_key: String = bs58::encode(keypair.to_bytes()).into_string();
    let public_key: String = keypair.pubkey().to_string();

    println!("Public Key: \x1b[34m{}\x1b[0m", public_key);
    println!("Private Key: \x1b[34m{}\x1b[0m", private_key);

    let env_content: String = format!("PRIVATE_KEY={}\nPUBLIC_KEY={}\n", private_key, public_key);
    fs::write("./.env", env_content).expect("Failed to write to .env file");

    println!("✅ The keys saved to .env file");
}
