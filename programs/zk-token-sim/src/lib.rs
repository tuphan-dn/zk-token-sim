use anchor_lang::prelude::*;

pub mod errors;
pub use errors::*;
pub mod schema;
pub use schema::*;
pub mod instructions;
pub use instructions::*;

declare_id!("nynPTAntJVuD4cZYmG1sAPu6voznhRKN9yeA7ZKfnEy");

#[program]
pub mod zk_token_sim {
  use super::*;

  pub fn initialize_mint(
    ctx: Context<InitializeMint>,
    supply_commitment: Pubkey,
    supply_decryption_handle: Pubkey,
  ) -> Result<()> {
    initialize_mint::exec(ctx, supply_commitment, supply_decryption_handle)
  }

  pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
    initialize_account::exec(ctx)
  }

  pub fn mint_to(
    ctx: Context<MintTo>,
    amount_commitment: Pubkey,
    amount_decryption_handle: Pubkey,
  ) -> Result<()> {
    mint_to::exec(ctx, amount_commitment, amount_decryption_handle)
  }

  pub fn transfer(
    ctx: Context<Transfer>,
    amount_commitment: Pubkey,
    amount_decryption_handle: Pubkey,
  ) -> Result<()> {
    transfer::exec(ctx, amount_commitment, amount_decryption_handle)
  }
}
