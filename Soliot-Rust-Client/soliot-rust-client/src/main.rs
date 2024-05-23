use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
    system_program,
};
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use tokio::time::{sleep, Duration};
use std::str::FromStr;
use std::path::Path;
use sha2::{Sha256, Digest};

const PROGRAM_ID: &str = "6ARFKMRmCWx9tcEiz1DWrZgJoWa5ErbWvcn4pBYt9J8C";
const REGISTRY_SEED: &[u8] = b"registry";
const NODE_SEED: &[u8] = b"node";
const TOKEN_MINT_ADDRESS: &str = "FPwdoxbJjhDGbWWAkK1vwqtvHr5EqbwkgWBaVB9UJ6Rx";

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let keypair_path = Path::new("/home/ritikbhatt020/.config/solana/id.json");
    let keypair = read_keypair_file(keypair_path).expect("Failed to read keypair file");

    let mint_authority_path = Path::new("/home/ritikbhatt020/.config/solana/id.json");
    let mint_authority = read_keypair_file(mint_authority_path).expect("Failed to read mint authority keypair file");

    if let Err(e) = initialize_registry(&client, &keypair, &mint_authority).await {
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

async fn initialize_registry(client: &RpcClient, keypair: &Keypair, _mint_authority: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (registry_pda, _bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);

    println!("Registry PDA: {}", registry_pda);
    println!("Mint Address: {}", TOKEN_MINT_ADDRESS);

    let mint_pubkey = Pubkey::from_str(TOKEN_MINT_ADDRESS)?;

    let rent_pubkey = solana_sdk::sysvar::rent::id();

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("initialize_registry"));

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(mint_pubkey, false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_token::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(rent_pubkey, false),
        ],
        data,
    };

    println!("Instruction: {:?}", instruction);

    send_transaction(client, keypair, vec![instruction], &[]).await?;
    println!("Registry initialized");
    Ok(())
}

async fn create_associated_token_account_if_not_exists(
    client: &RpcClient,
    keypair: &Keypair,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let associated_token_account = get_associated_token_address(owner, mint);
    let account_info = client.get_account(&associated_token_account);

    if account_info.is_err() {
        let create_ata_ix = create_associated_token_account(
            &keypair.pubkey(),  // Payer
            owner,  // Owner
            mint,
            &TOKEN_PROGRAM_ID,  // Token program ID
        );
        let recent_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[create_ata_ix],
            Some(&keypair.pubkey()),
            &[keypair],
            recent_blockhash,
        );
        client.send_and_confirm_transaction(&tx)?;
        println!("Associated Token Account created: {}", associated_token_account);
    } else {
        println!("Associated Token Account already exists: {}", associated_token_account);
    }
    Ok(())
}

async fn register_node(client: &RpcClient, keypair: &Keypair, ip: [u8; 4]) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);
    let (registry_pda, _registry_bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);
    let token_mint_pubkey = Pubkey::from_str(TOKEN_MINT_ADDRESS)?;
    let node_token_account = get_associated_token_address(&keypair.pubkey(), &token_mint_pubkey);

    println!("Node PDA: {}", node_pda);
    println!("Registry PDA: {}", registry_pda);
    println!("Node Token Account: {}", node_token_account);

    // Ensure the associated token account exists
    create_associated_token_account_if_not_exists(client, keypair, &keypair.pubkey(), &token_mint_pubkey).await?;

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("register"));
    data.extend_from_slice(&ip);

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(node_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(node_token_account, false),
            solana_sdk::instruction::AccountMeta::new_readonly(token_mint_pubkey, false),
            solana_sdk::instruction::AccountMeta::new_readonly(system_program::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    };

    println!("Instruction: {:?}", instruction);

    send_transaction(client, keypair, vec![instruction], &[]).await?;
    println!("Node registered with IP: {:?}", ip);
    Ok(())
}

async fn update_node(
    client: &RpcClient,
    keypair: &Keypair,
    uptime: u64,
    heartbeat: u64,
    bytes: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let program_id = Pubkey::from_str(PROGRAM_ID)?;
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);
    let token_mint_pubkey = Pubkey::from_str(TOKEN_MINT_ADDRESS)?;
    let node_token_account = get_associated_token_address(&keypair.pubkey(), &token_mint_pubkey);

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("update"));
    data.extend_from_slice(&uptime.to_le_bytes());
    data.extend_from_slice(&heartbeat.to_le_bytes());
    data.extend_from_slice(&bytes.to_le_bytes());

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(node_pda, false), // Node PDA
            solana_sdk::instruction::AccountMeta::new_readonly(keypair.pubkey(), true), // Authority (Signer)
            solana_sdk::instruction::AccountMeta::new(token_mint_pubkey, false), // Mint
            solana_sdk::instruction::AccountMeta::new(node_token_account, false), // Node Token Account
            solana_sdk::instruction::AccountMeta::new_readonly(keypair.pubkey(), false), // Mint Authority (Not a signer, not writable)
            solana_sdk::instruction::AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // Token Program
        ],
        data,
    };

    send_transaction(client, keypair, vec![instruction], &[]).await?;
    println!("Node updated with uptime: {}, heartbeat: {}, bytes: {}", uptime, heartbeat, bytes);
    Ok(())
}


async fn send_transaction(
    client: &RpcClient,
    keypair: &Keypair,
    instructions: Vec<Instruction>,
    signers: &[&Keypair],
) -> Result<(), Box<dyn std::error::Error>> {
    let recent_blockhash = client.get_latest_blockhash()?;

    let mut all_signers: Vec<&Keypair> = vec![keypair];
    all_signers.extend_from_slice(signers);

    let mut transaction = Transaction::new_with_payer(&instructions, Some(&keypair.pubkey()));
    transaction.sign(&all_signers, recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);
    Ok(())
}
