use anchor_lang::{prelude::*};

#[account]
pub struct NftKind {
    /// Master edtion nft mint address
    pub nft_mint: Pubkey,

    /// MasterEdtion NFT kind
    pub nft_kind: u8, // 0: strength, 1: agility, 2: intelligence

    /// bump
    pub bump: u8
}

#[account]
pub struct NftAttribute {
    /// user wallet
    pub user_wallet: Pubkey,

    /// nft mint address
    pub nft_mint: Pubkey,

    /// health
    pub health: u32,

    /// mana
    pub mana: u32,

    /// power
    pub power: u32,

    /// nft kind
    pub nft_kind: u8, // 0: strength, 1: agility, 2: intelligence

    /// bump
    pub bump: u8
}

#[account]
pub struct StakeInfo {
    /// user wallet
    pub user_wallet: Pubkey,

    /// nft mint address
    pub nft_mint: Pubkey,

    /// bump
    pub bump: u8
}

#[account]
pub struct ReputationPointInfo {
    /// user wallet
    pub user_wallet: Pubkey,

    /// reputation point
    pub reputation_point: u32,

    /// bump
    pub bump: u8
}