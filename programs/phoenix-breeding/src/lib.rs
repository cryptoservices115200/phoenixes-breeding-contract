
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

pub mod account;
pub mod constants;
pub mod error;
pub mod utils;

use account::*;
use constants::*;
use error::*;
use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod phoenix_breeding {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.global_state.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn breed(ctx: Context<Breed>) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn evolve(ctx: Context<Evolve>) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Breed<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user
    )]
    pub breeding_nft_data: Account<'info, BreedingNftInfo>,

    pub lava_mint: Account<'info, Mint>,

    pub user_lava_ata: Account<'info, TokenAccount>,

    pub breed_nft_mint: Account<'info, Mint>,

    pub breed_nft_token_acc: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_STATE_SEED.as_ref()],
        bump,
        payer = admin
    )]
    pub global_state: Account<'info, GlobalState>,

    pub lava_mint: Account<'info, Mint>,
    
    pub breed_nft_mint: Account<'info, Mint>,
    
    pub evolve_nft_mint: Account<'info, Mint>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Evolve <'info>{
        
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user
    )]
    pub evolving_nft_data: Account<'info, EvolvingNftInfo>,

    pub lava_mint: Account<'info, Mint>,

    pub user_lava_ata: Account<'info, TokenAccount>,

    pub evolve_nft_mint: Account<'info, Mint>,

    pub evolve_nft_token_acc: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
