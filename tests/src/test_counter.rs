use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};

#[test]
fn initialize_counter() {
    let program_id = "YahmvUUTTjfWTvtBd4PVUow8H6sg1aVvNTkgrXHnn2a";
    let anchor_wallet = "/home/gidon/.config/solana/id.json".to_string();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(counter_anchor::accounts::InitializeCounter {})
        .args(counter_anchor::instruction::InitializeCounter {})
        .send()
        .expect("Failed to send transaction");

    println!("Your transaction signature {}", tx);
}
