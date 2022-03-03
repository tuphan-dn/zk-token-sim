use crate::schema::{account, mint};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
  pub mint: Account<'info, mint::Mint>,
  #[account(
    init,
    payer = authority,
    space = account::Account::LEN,
    seeds = [
      b"account".as_ref(),
      &mint.key().to_bytes(),
      &owner.key().to_bytes(),
    ],
    bump
  )]
  pub account: Account<'info, account::Account>,
  #[account(mut)]
  pub authority: Signer<'info>,
  pub owner: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(ctx: Context<InitializeAccount>) -> Result<()> {
  let account = &mut ctx.accounts.account;
  account.authority = ctx.accounts.owner.key();
  Ok(())
}
