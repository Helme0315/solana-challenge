use anchor_lang::{prelude::*};
use crate::constants::*;
use crate::states::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitUserReputation<'info> {
    /// user wallet
    #[account(mut)]
    pub user_wallet: Signer<'info>,

    // create PDA to calculate the reputation point of each user
    #[account(
        init,
        seeds = [
            USER_REPUTATION_INFO.as_ref(),
            user_wallet.key().as_ref()
        ],
        bump,
        payer = user_wallet,
        space = 8 + size_of::<ReputationPointInfo>(),
    )]
    pub reputation_info: Box<Account<'info, ReputationPointInfo>>,

    /// system program
    pub system_program: Program<'info, System>,
}


impl<'info> InitUserReputation<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {

        self.reputation_info.user_wallet = self.user_wallet.key();
        self.reputation_info.reputation_point = 0;
        self.reputation_info.bump = bump;
        Ok(())
    }
}