use crate::schema::{account, mint};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintTo<'info> {
  #[account(mut, has_one = authority)]
  pub mint: Account<'info, mint::Mint>,
  #[account(
    init_if_needed,
    payer = authority,
    space = account::Account::LEN,
    seeds = [
      b"account".as_ref(),
      &mint.key().to_bytes(),
      &owner.key().to_bytes(),
    ],
    bump
  )]
  pub associated_token_account: Account<'info, account::Account>,
  #[account(mut)]
  pub authority: Signer<'info>,
  /// CHECK: pure account
  pub owner: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn exec(
  ctx: Context<MintTo>,
  amount_commitment: Pubkey,
  amount_decryption_handle: Pubkey,
) -> Result<()> {
  // Update mint
  let mint = &mut ctx.accounts.mint;
  mint.mint_to(amount_commitment, amount_decryption_handle);
  // Update account
  let associated_token_account = &mut ctx.accounts.associated_token_account;
  associated_token_account.authority = ctx.accounts.owner.key();
  associated_token_account.add(amount_commitment, amount_decryption_handle);
  Ok(())
}
