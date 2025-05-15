use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};

#[test]
fn test_initialize() {
    let program_id = "7MtMDbYQxhjkqWPoZT1szVSeWwaGUMUJzYnHMs1jD8Wq";
    let anchor_wallet = "/home/gidon/.config/solana/id.json".to_string();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let tx = program
        .request()
        .accounts(solana_program_with_anchor::accounts::Initialize {})
        .args(solana_program_with_anchor::instruction::Initialize {})
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
