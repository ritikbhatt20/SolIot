use anchor_lang::prelude::*;

mod constants;
mod error;

use crate::{constants::*, error::*};

declare_id!("7ThjD7Yb8TjMKzw8xZjJDJkMzaVKmo7diSX56mNVSxsi");

#[program]
pub mod iot_devices {
    use super::*;

    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.nodes = Vec::new();
        Ok(())
    }

    pub fn register(ctx: Context<Register>, ip: [u8; 4]) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.node_pubkey = *ctx.accounts.authority.key;
        node.authority = *ctx.accounts.authority.key;
        node.ip = ip;                                        
        node.uptime = 0;                                 
        node.heartbeat = 0;
        node.bytes = 0;

        let registry = &mut ctx.accounts.registry;
        registry.nodes.push(*ctx.accounts.node.to_account_info().key);
        Ok(())
    }   

    pub fn update(ctx: Context<Update>, uptime: u64, heartbeat: u64, bytes: u64) -> Result<()> {
        let node = &mut ctx.accounts.node;
        node.uptime = uptime;
        node.heartbeat = heartbeat;
        node.bytes = bytes;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(
        init,
        payer = user, 
        space = 8 + 1000, 
        seeds = [REGISTRY_SEED.as_bytes()], 
        bump
    )]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        init,
        payer = authority, 
        space = 8 + 1000, 
        seeds = [NODE_SEED.as_bytes(), authority.key().as_ref()], 
        bump
    )] 
    pub node: Account<'info, NodeStatus>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [REGISTRY_SEED.as_bytes()], bump)]
    pub registry: Account<'info, Registry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority, seeds = [NODE_SEED.as_bytes(), authority.key().as_ref()], bump)]
    pub node: Account<'info, NodeStatus>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct NodeStatus {
    pub node_pubkey: Pubkey,                    
    pub authority: Pubkey,
    pub ip: [u8; 4], 
    pub uptime: u64,
    pub heartbeat: u64,
    pub bytes: u64,
}

#[account]
pub struct Registry {
    pub nodes: Vec<Pubkey>,
}