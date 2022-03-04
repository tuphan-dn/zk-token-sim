use anchor_lang::prelude::*;

pub mod errors;
pub use errors::*;
pub mod schema;
pub use schema::*;
pub mod instructions;
pub use instructions::*;

declare_id!("8uJfchuep2PKUG8TUmqe1gneUMwcu5fkTtC7yoCvMVvd");

#[derive(Accounts)]
pub struct Test {}

#[program]
pub mod zk_token_sim {
  use super::*;

  pub fn initialize_mint(
    ctx: Context<Test>,
    supply_commitment: Point,
    supply_decryption_handle: Point,
  ) -> Result<()> {
    // initialize_mint::exec(ctx, supply_commitment, supply_decryption_handle)
    Ok(())
  }

  // pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
  //   initialize_account::exec(ctx)
  // }

  // pub fn mint_to(
  //   ctx: Context<MintTo>,
  //   amount_commitment: Point,
  //   amount_decryption_handle: Point,
  // ) -> Result<()> {
  //   mint_to::exec(ctx, amount_commitment, amount_decryption_handle)
  // }

  // pub fn transfer(
  //   ctx: Context<Transfer>,
  //   amount_commitment: Point,
  //   amount_decryption_handle: Point,
  // ) -> Result<()> {
  //   transfer::exec(ctx, amount_commitment, amount_decryption_handle)
  // }
}
