use crate::schema::mint;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeMint<'info> {
  #[account(init, payer = authority, space = mint::Mint::LEN)]
  pub mint: Account<'info, mint::Mint>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(
  ctx: Context<InitializeMint>,
  supply_commitment: Pubkey,
  supply_decryption_handle: Pubkey,
) -> Result<()> {
  let mint = &mut ctx.accounts.mint;
  mint.authority = ctx.accounts.authority.key();
  mint.supply_commitment = supply_commitment;
  mint.supply_decryption_handle = supply_decryption_handle;
  Ok(())
}
