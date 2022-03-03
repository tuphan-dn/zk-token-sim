use crate::point::Point;
use anchor_lang::prelude::*;

/**
 * Mint struct
 */
#[account]
pub struct Mint {
  pub authority: Pubkey,
  pub supply_commitment: Point,
  pub supply_decryption_handle: Point,
}

impl Mint {
  pub const LEN: usize = 8 + 32 + 32 + 32;

  pub fn mint(
    &mut self,
    supply_commitment: Point,
    supply_decryption_handle: Point,
  ) -> Option<(Point, Point)> {
    self.supply_commitment = self.supply_commitment.add(supply_commitment);
    self.supply_decryption_handle = self.supply_decryption_handle.add(supply_decryption_handle);
    return Some((self.supply_commitment, self.supply_decryption_handle));
  }
}
