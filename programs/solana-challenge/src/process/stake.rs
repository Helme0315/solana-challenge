use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::*;
use crate::states::*;

use std::mem::size_of;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user_wallet: Signer<'info>,

    /// edtion nft mint
    pub nft_mint: Box<Account<'info, Mint>>,

    /// edtion tnft oken account
    #[account(
        mut,
        constraint = user_token_account.mint == nft_mint.key() @ Errors::InvalidTokenMint,
        constraint = user_token_account.owner == user_wallet.key() @ Errors::InvalidTokenOwner
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Safe account
    #[account(
        seeds = [
            NFT_AUTHORITY.as_ref()
        ],
        bump
    )]
    pub nft_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = nft_authority_token_account.mint == nft_mint.key() @ Errors::InvalidTokenMint,
        constraint = nft_authority_token_account.owner == nft_authority.key() @ Errors::InvalidTokenOwner
    )]
    pub nft_authority_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [
            NFT_STAKE.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        payer = user_wallet,
        space = 8 + size_of::<StakeInfo>(),
    )]
    pub stake_info: Box<Account<'info, StakeInfo>>,

    #[account(
        seeds = [
            ATTRIBUTE.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump = nft_attribute.bump,
        constraint = nft_attribute.user_wallet == user_wallet.key() @ Errors::AccessDenied
    )]
    pub nft_attribute: Box<Account<'info, NftAttribute>>,

    #[account(
        mut,
        seeds = [
            USER_REPUTATION_INFO.as_ref(),
            user_wallet.key().as_ref()
        ],
        bump = reputation_info.bump,
        constraint = reputation_info.user_wallet == user_wallet.key() @ Errors::AccessDenied
    )]
    pub reputation_info: Box<Account<'info, ReputationPointInfo>>,

    /// system program
    pub system_program: Program<'info, System>,

    /// token program
    pub token_program: Program<'info, Token>,
}


impl<'info> Stake<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {

        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.user_token_account.to_account_info(),
                    to: self.nft_authority_token_account.to_account_info(),
                    authority: self.user_wallet.to_account_info(),
                },
            ),
            1,
        )?;

        let mut reputation_point = 0;

        match self.nft_attribute.nft_kind {
            0 => {
                // strength nft
                reputation_point = (self.nft_attribute.health*2).checked_add(self.nft_attribute.mana).unwrap().checked_add(self.nft_attribute.power).unwrap();
            },
            1 => {
                // agility nft
                reputation_point = self.nft_attribute.health.checked_add(self.nft_attribute.mana).unwrap().checked_add(self.nft_attribute.power*2).unwrap();
            },
            2 => {
                // inteligence nft
                reputation_point = (self.nft_attribute.health*2).checked_add(self.nft_attribute.mana*2).unwrap().checked_add(self.nft_attribute.power).unwrap();
            },
            _ => {
                return Err(error!(Errors::InvalidNftKind));
            }
        }

        self.stake_info.user_wallet = self.user_wallet.key();
        self.stake_info.nft_mint = self.nft_mint.key();
        self.stake_info.bump = bump;

        self.reputation_info.reputation_point = self.reputation_info.reputation_point.checked_add(reputation_point).unwrap();

        Ok(())
    }
}
