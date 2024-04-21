pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Er16QUB2BFjkcEoJ2mg9hWbeumTA8zbcFRhcsHPBQQ3K");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn createmint(ctx: Context<CreateMint>,jelly_uri: String, usdc_uri: String, nft_uri: String, jelly_name: String, usdc_name: String, nft_name: String, jelly_symbol: String, usdc_symbol: String, nft_symbol: String) -> Result<()>{
        instructions::createmint(ctx, jelly_uri, usdc_uri, nft_uri, jelly_name, usdc_name, nft_name, jelly_symbol, usdc_symbol, nft_symbol)
    }

    // // pay usdt to buy our tokens
    // pub fn buytokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    //     instructions::buytokens(ctx, amount)
    // }

    // // pay amount token to get usdt return
    // pub fn usetokens(ctx: Context<UseTokens>, amount: u64) -> Result<()> {
    //     instructions::usetokens(ctx, amount)
    // }
}

