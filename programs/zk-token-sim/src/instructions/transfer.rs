use crate::point::Point;
use crate::schema::{account, mint};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Transfer<'info> {
  pub mint: Account<'info, mint::Mint>,
  #[account(mut)]
  pub src_associated_token_account: Account<'info, account::Account>,
  #[account(
    init_if_needed,
    payer = src_owner,
    space = account::Account::LEN,
    seeds = [
      b"account".as_ref(),
      &mint.key().to_bytes(),
      &dst_owner.key().to_bytes(),
    ],
    bump
  )]
  pub dst_associated_token_account: Account<'info, account::Account>,
  #[account(mut)]
  pub src_owner: Signer<'info>,
  /// CHECK: pure account
  pub dst_owner: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(
  ctx: Context<Transfer>,
  amount_commitment: Point,
  amount_decryption_handle: Point,
) -> Result<()> {
  let src_associated_token_account = &mut ctx.accounts.src_associated_token_account;
  src_associated_token_account.sub(amount_commitment, amount_decryption_handle);
  let dst_associated_token_account = &mut ctx.accounts.dst_associated_token_account;
  dst_associated_token_account.add(amount_commitment, amount_decryption_handle);
  Ok(())
}
