use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}
