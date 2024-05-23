use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

mod error;

use crate::error::*;

declare_id!("RCw9Jvc35yuko59KzBtmSfRkZrd6aPWzioEyioXyqmk");

#[program]
pub mod iot_devices {
    use super::*;

    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.nodes = Vec::new();
        registry.token_mint = ctx.accounts.mint.key();
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
        node.last_update_slot = 0;
        node.token_earnings = 0;
        
        // Manually derive the bump
        let (_node_key, bump) = Pubkey::find_program_address(
            &[b"node", ctx.accounts.authority.key().as_ref()],
            ctx.program_id
        );
        node.bump = bump;

        let registry = &mut ctx.accounts.registry;
        registry.nodes.push(*ctx.accounts.node.to_account_info().key);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, uptime: u64, heartbeat: u64, bytes: u64) -> Result<()> {
        // Immutable borrows before mutable borrow
        let node_key = *ctx.accounts.node.to_account_info().key;
        let node_bump = ctx.accounts.node.bump;
        let current_slot = Clock::get()?.slot;

        {
            let node = &mut ctx.accounts.node;

            if node.last_update_slot == current_slot {
                return err!(IotError::UpdateAlreadyCalled);
            }

            // Mutable borrows
            node.uptime = uptime;
            node.heartbeat = heartbeat;
            node.bytes = bytes;
            node.last_update_slot = current_slot;
        }

        // Mint tokens to the node's token account
        let seeds = &[node_key.as_ref(), &[node_bump]];
        let signer = &[&seeds[..]];
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info().clone(),
            to: ctx.accounts.node_token_account.to_account_info().clone(),
            authority: ctx.accounts.mint_authority.to_account_info().clone(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::mint_to(cpi_ctx, 1)?;

        {
            let node = &mut ctx.accounts.node;
            // Update token earnings
            node.token_earnings += 1;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = user, space = 8 + 1000, seeds = [b"registry"], bump)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(init, payer = authority, space = 8 + 1000, seeds = [b"node", authority.key().as_ref()], bump)]
    pub node: Account<'info, NodeStatus>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"registry"], bump)]
    pub registry: Account<'info, Registry>,
    #[account(init_if_needed, payer = authority, associated_token::mint = mint, associated_token::authority = authority)]
    pub node_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = authority, seeds = [b"node", authority.key().as_ref()], bump)]
    pub node: Account<'info, NodeStatus>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub node_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct NodeStatus {
    pub node_pubkey: Pubkey,
    pub authority: Pubkey,
    pub ip: [u8; 4],
    pub uptime: u64,
    pub heartbeat: u64,
    pub bytes: u64,
    pub last_update_slot: u64,
    pub token_earnings: u64,
    pub bump: u8,
}

#[account]
pub struct Registry {
    pub nodes: Vec<Pubkey>,
    pub token_mint: Pubkey,
}
