use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use tokio::time::{sleep, Duration};
use std::str::FromStr;
use std::path::Path;
use sha2::{Sha256, Digest};

const PROGRAM_ID: &str = "7ThjD7Yb8TjMKzw8xZjJDJkMzaVKmo7diSX56mNVSxsi";
const REGISTRY_SEED: &[u8] = b"registry";
const NODE_SEED: &[u8] = b"node";

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let keypair_path = Path::new(r"/home/ritikbhatt020/.config/solana/id.json");
    let keypair = read_keypair_file(keypair_path).expect("Failed to read keypair file");

    if let Err(e) = initialize_registry(&client, &keypair).await {
        eprintln!("Failed to initialize registry: {:?}", e);
        return;
    }

    let ip = [192, 168, 1, 1];
    if let Err(e) = register_node(&client, &keypair, ip).await {
        eprintln!("Failed to register node: {:?}", e);
        return;
    }

    loop {
        let uptime = 100;
        let heartbeat = 5;
        let bytes = 1024;
        if let Err(e) = update_node(&client, &keypair, uptime, heartbeat, bytes).await {
            eprintln!("Failed to update node: {:?}", e);
        }
        sleep(Duration::from_secs(600)).await;
    }
}

fn get_discriminator(instruction_name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", instruction_name));
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

async fn initialize_registry(client: &RpcClient, keypair: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (registry_pda, _bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("initialize_registry"));

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(solana_sdk::system_program::id(), false),
        ],
        data,
    };

    send_transaction(client, keypair, instruction).await?;
    println!("Registry initialized");
    Ok(())
}

async fn register_node(client: &RpcClient, keypair: &Keypair, ip: [u8; 4]) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (node_pda, _bump) = Pubkey::find_program_address(
        &[NODE_SEED, keypair.pubkey().as_ref()],
        &program_id,
    );
    let (registry_pda, _bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("register"));
    data.extend_from_slice(&ip);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(node_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(solana_sdk::system_program::id(), false),
        ],
        data,
    };

    send_transaction(client, keypair, instruction).await?;
    println!("Node registered with IP: {:?}", ip);
    Ok(())
}

async fn update_node(client: &RpcClient, keypair: &Keypair, uptime: u64, heartbeat: u64, bytes: u64) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (node_pda, _bump) = Pubkey::find_program_address(
        &[NODE_SEED, keypair.pubkey().as_ref()],
        &program_id,
    );

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("update"));
    data.extend_from_slice(&uptime.to_le_bytes());
    data.extend_from_slice(&heartbeat.to_le_bytes());
    data.extend_from_slice(&bytes.to_le_bytes());

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(node_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
        ],
        data,
    };

    send_transaction(client, keypair, instruction).await?;
    println!(
        "Node updated with uptime: {}, heartbeat: {}, bytes: {}",
        uptime, heartbeat, bytes
    );
    Ok(())
}

async fn send_transaction(client: &RpcClient, keypair: &Keypair, instruction: Instruction) -> Result<(), Box<dyn std::error::Error>> {
    let message = Message::new(&[instruction], Some(&keypair.pubkey()));
    let recent_blockhash = client.get_recent_blockhash()?.0;
    let tx = Transaction::new(&[keypair], message, recent_blockhash);

    client.send_and_confirm_transaction(&tx)?;
    Ok(())
}