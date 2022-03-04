use crate::point::Point;
use anchor_lang::prelude::*;

/**
 * Account struct
 */
#[account]
pub struct Account {
  pub authority: Pubkey,
  pub amount_commitment: Pubkey,
  pub amount_decryption_handle: Pubkey,
}

impl Account {
  pub const LEN: usize = 8 + 32 + 32 + 32;

  pub fn add(
    &mut self,
    amount_commitment: Pubkey,
    amount_decryption_handle: Pubkey,
  ) -> Option<(Pubkey, Pubkey)> {
    let prev_ac = Point::new_from_pubkey(self.amount_commitment);
    let ac = Point::new_from_pubkey(amount_commitment);
    let prev_ad = Point::new_from_pubkey(self.amount_decryption_handle);
    let ad = Point::new_from_pubkey(amount_decryption_handle);
    let nxt_ac = prev_ac.add(ac);
    let nxt_ad = prev_ad.add(ad);
    return Some((nxt_ac.to_pubkey(), nxt_ad.to_pubkey()));
  }
  pub fn sub(
    &mut self,
    amount_commitment: Pubkey,
    amount_decryption_handle: Pubkey,
  ) -> Option<(Pubkey, Pubkey)> {
    let prev_ac = Point::new_from_pubkey(self.amount_commitment);
    let ac = Point::new_from_pubkey(amount_commitment);
    let prev_ad = Point::new_from_pubkey(self.amount_decryption_handle);
    let ad = Point::new_from_pubkey(amount_decryption_handle);
    let nxt_ac = prev_ac.sub(ac);
    let nxt_ad = prev_ad.sub(ad);
    return Some((nxt_ac.to_pubkey(), nxt_ad.to_pubkey()));
  }
}
