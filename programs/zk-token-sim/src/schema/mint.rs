use crate::point::Point;
use anchor_lang::prelude::*;

/**
 * Mint struct
 */
#[account]
#[derive(Debug)]
pub struct Mint {
  pub authority: Pubkey,
  pub supply_commitment: Pubkey,
  pub supply_decryption_handle: Pubkey,
}

impl Mint {
  pub const LEN: usize = 8 + 32 + 32 + 32;

  pub fn mint_to(
    &mut self,
    supply_commitment: Pubkey,
    supply_decryption_handle: Pubkey,
  ) -> Option<(Pubkey, Pubkey)> {
    let prev_sc = Point::new_from_pubkey(self.supply_commitment);
    let sc = Point::new_from_pubkey(supply_commitment);
    let prev_sd = Point::new_from_pubkey(self.supply_decryption_handle);
    let sd = Point::new_from_pubkey(supply_decryption_handle);
    let nxt_sc = prev_sc.add(sc);
    let nxt_sd = prev_sd.add(sd);
    return Some((nxt_sc.to_pubkey(), nxt_sd.to_pubkey()));
  }
}
