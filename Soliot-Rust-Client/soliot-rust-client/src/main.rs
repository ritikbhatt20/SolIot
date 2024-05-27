use dotenv::dotenv;
use std::env;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
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
use std::io;

const REGISTRY_SEED: &[u8] = b"registry";
const NODE_SEED: &[u8] = b"node";

#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let client = RpcClient::new(rpc_url);

    let keypair_path_str = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
    let keypair_path = Path::new(&keypair_path_str);
    let keypair = read_keypair_file(keypair_path).expect("Failed to read keypair file");

    let mint_authority_path_str = env::var("MINT_AUTHORITY_PATH").expect("MINT_AUTHORITY_PATH must be set");
    let mint_authority_path = Path::new(&mint_authority_path_str);
    let mint_authority = read_keypair_file(mint_authority_path).expect("Failed to read mint authority keypair file");

    let program_id_str = env::var("PROGRAM_ID").expect("PROGRAM_ID must be set");
    let program_id = Pubkey::from_str(&program_id_str).expect("Invalid PROGRAM_ID");
    
    let token_mint_address_str = env::var("TOKEN_MINT_ADDRESS").expect("TOKEN_MINT_ADDRESS must be set");
    let token_mint_address = Pubkey::from_str(&token_mint_address_str).expect("Invalid TOKEN_MINT_ADDRESS");

    if let Err(e) = initialize_registry_if_needed(&client, &keypair, &mint_authority, program_id, token_mint_address).await {
        eprintln!("Failed to initialize registry: {:?}", e);
    }

    let node_registered = check_node_registration(&client, &keypair, program_id).await;

    if !node_registered {
        println!("Enter the IP address (format: x.x.x.x):");
        let ip = read_ip();

        if let Err(e) = register_node_if_needed(&client, &keypair, ip, program_id, token_mint_address).await {
            eprintln!("Failed to register node: {:?}", e);
        }
    } else {
        println!("Node already registered");
    }

    loop {
        println!("Enter uptime:");
        let uptime = read_u64();

        println!("Enter heartbeat:");
        let heartbeat = read_u64();

        println!("Enter bytes:");
        let bytes = read_u64();

        if let Err(e) = update_node(&client, &keypair, uptime, heartbeat, bytes, program_id, token_mint_address).await {
            eprintln!("Failed to update node: {:?}", e);
        }

        println!("Do you want to unregister the node? (yes/no):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "yes" {
            if let Err(e) = unregister_node(&client, &keypair, program_id).await {
                eprintln!("Failed to unregister node: {:?}", e);
            }
            break;
        }
        
        sleep(Duration::from_secs(600)).await;
    }
}

async fn check_node_registration(client: &RpcClient, keypair: &Keypair, program_id: Pubkey) -> bool {
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);
    client.get_account(&node_pda).is_ok()
}

fn read_ip() -> [u8; 4] {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split('.').collect();
        if parts.len() == 4 {
            if let Ok(ip) = parts.iter().map(|&part| part.parse::<u8>()).collect::<Result<Vec<u8>, _>>() {
                if ip.len() == 4 {
                    return [ip[0], ip[1], ip[2], ip[3]];
                }
            }
        }
        println!("Invalid IP address format. Please enter in the format x.x.x.x:");
    }
}

fn read_u64() -> u64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(value) = input.trim().parse::<u64>() {
            return value;
        }
        println!("Invalid input. Please enter a valid number:");
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

async fn initialize_registry_if_needed(client: &RpcClient, keypair: &Keypair, _mint_authority: &Keypair, program_id: Pubkey, token_mint_address: Pubkey) -> Result<(), Box<dyn std::error::Error>> {
    let (registry_pda, _bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);

    if client.get_account(&registry_pda).is_ok() {
        println!("Registry already initialized");
        return Ok(());
    }

    let rent_pubkey = solana_sdk::sysvar::rent::id();

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("initialize_registry"));

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(token_mint_address, false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_token::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(rent_pubkey, false),
        ],
        data,
    };

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
            &keypair.pubkey(),
            owner,
            mint,
            &TOKEN_PROGRAM_ID,
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

async fn register_node_if_needed(client: &RpcClient, keypair: &Keypair, ip: [u8; 4], program_id: Pubkey, token_mint_address: Pubkey) -> Result<(), Box<dyn std::error::Error>> {
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);

    if client.get_account(&node_pda).is_ok() {
        println!("Node already registered");
        return Ok(());
    }

    let (registry_pda, _registry_bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);
    let node_token_account = get_associated_token_address(&keypair.pubkey(), &token_mint_address);

    create_associated_token_account_if_not_exists(client, keypair, &keypair.pubkey(), &token_mint_address).await?;

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
            solana_sdk::instruction::AccountMeta::new_readonly(token_mint_address, false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    };

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
    program_id: Pubkey,
    token_mint_address: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);
    let node_token_account = get_associated_token_address(&keypair.pubkey(), &token_mint_address);

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
            solana_sdk::instruction::AccountMeta::new(token_mint_address, false), // Mint
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

async fn unregister_node(client: &RpcClient, keypair: &Keypair, program_id: Pubkey) -> Result<(), Box<dyn std::error::Error>> {
    let (node_pda, _node_bump) = Pubkey::find_program_address(&[NODE_SEED, keypair.pubkey().as_ref()], &program_id);

    let (registry_pda, _registry_bump) = Pubkey::find_program_address(&[REGISTRY_SEED], &program_id);

    let mut data = Vec::new();
    data.extend_from_slice(&get_discriminator("unregister"));

    let instruction = Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(node_pda, false),
            solana_sdk::instruction::AccountMeta::new(keypair.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new(registry_pda, false),
        ],
        data,
    };

    send_transaction(client, keypair, vec![instruction], &[]).await?;
    println!("Node unregistered");
    Ok(())
}

async fn send_transaction(
    client: &RpcClient,
    keypair: &Keypair,
    instructions: Vec<Instruction>,
    signers: &[&Keypair],
) -> Result<(), Box<dyn std::error::Error>> {
    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&keypair.pubkey()),
        &signers.iter().chain(std::iter::once(&keypair)).copied().collect::<Vec<_>>(),
        recent_blockhash,
    );
    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Transaction signature: {}", signature);
    Ok(())
}