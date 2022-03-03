use crate::point::Point;
use anchor_lang::prelude::*;

/**
 * Account struct
 */
#[account]
pub struct Account {
  pub authority: Pubkey,
  pub amount_commitment: Point,
  pub amount_decryption_handle: Point,
}

impl Account {
  pub const LEN: usize = 8 + 32 + 32 + 32;

  pub fn add(
    &mut self,
    amount_commitment: Point,
    amount_decryption_handle: Point,
  ) -> Option<(Point, Point)> {
    self.amount_commitment = self.amount_commitment.add(amount_commitment);
    self.amount_decryption_handle = self.amount_decryption_handle.add(amount_decryption_handle);
    return Some((self.amount_commitment, self.amount_decryption_handle));
  }
  pub fn sub(
    &mut self,
    amount_commitment: Point,
    amount_decryption_handle: Point,
  ) -> Option<(Point, Point)> {
    self.amount_commitment = self.amount_commitment.sub(amount_commitment);
    self.amount_decryption_handle = self.amount_decryption_handle.sub(amount_decryption_handle);
    return Some((self.amount_commitment, self.amount_decryption_handle));
  }
}
