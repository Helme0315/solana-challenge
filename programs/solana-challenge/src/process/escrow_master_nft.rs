use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::constants::*;
use crate::error::*;
use crate::states::*;

use std::mem::size_of;

#[derive(Accounts)]
pub struct EscrowMasterNft<'info> {
    // admin wallet to escrow the MasterEdition NFT 
    #[account(mut)]
    pub admin: Signer<'info>,

    /// master edtion nft mint
    pub nft_mint: Box<Account<'info, Mint>>,

    /// master edtion token account
    #[account(
        mut,
        constraint = admin_token_account.mint == nft_mint.key() @ Errors::InvalidTokenMint,
        constraint = admin_token_account.owner == admin.key() @ Errors::InvalidTokenOwner
    )]
    pub admin_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Safe account
    #[account(
        seeds = [
            NFT_AUTHORITY.as_ref()
        ],
        bump
    )]
    pub nft_authority: UncheckedAccount<'info>,

    // MasterEdition NFT token account of PDA
    #[account(
        mut,
        constraint = nft_authority_token_account.mint == nft_mint.key() @ Errors::InvalidTokenMint,
        constraint = nft_authority_token_account.owner == nft_authority.key() @ Errors::InvalidTokenOwner
    )]
    pub nft_authority_token_account: Box<Account<'info, TokenAccount>>,

    // create PDA to save the NFT Kind; strength, agility, intelligence
    #[account(
        init,
        seeds = [
            NFT_KIND.as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<NftKind>(),
    )]
    pub nft_type: Box<Account<'info, NftKind>>,

    /// system program
    pub system_program: Program<'info, System>,

    /// token program
    pub token_program: Program<'info, Token>,
}


impl<'info> EscrowMasterNft<'info> {
    pub fn process(&mut self, bump: u8, args: EscrowMasterNftArgs) -> Result<()> {

        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.admin_token_account.to_account_info(),
                    to: self.nft_authority_token_account.to_account_info(),
                    authority: self.admin.to_account_info(),
                },
            ),
            1,
        )?;

        self.nft_type.nft_mint = self.nft_mint.key();
        self.nft_type.nft_kind = args.kind;
        self.nft_type.bump = bump;

        Ok(())
    }
}



#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EscrowMasterNftArgs {
    /// master edtion nft kind
    pub kind: u8, // 0: strength, 1: agility, 2: intelligence
}