use anchor_lang::prelude::*;
use core::ops::{Add, Mul, Sub};
use curve25519_dalek::{
  constants::{ED25519_BASEPOINT_COMPRESSED, ED25519_BASEPOINT_POINT},
  edwards::{CompressedEdwardsY, EdwardsPoint},
  scalar::Scalar,
};
use lazy_static::lazy_static;
use sha3::Sha3_512;

lazy_static! {
  /// Pedersen base point for encoding the commitment openings.
  pub static ref H: EdwardsPoint = EdwardsPoint::hash_from_bytes::<Sha3_512>(ED25519_BASEPOINT_COMPRESSED.as_bytes());
}

/**
 * EC Point
 */
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point(pub(crate) EdwardsPoint);

impl Point {
  pub fn get_base() -> Self {
    Self(ED25519_BASEPOINT_POINT)
  }
  pub fn get_infinity() -> Self {
    Self(ED25519_BASEPOINT_POINT.sub(ED25519_BASEPOINT_POINT))
  }

  pub fn new_from_pubkey(pubkey: Pubkey) -> Self {
    let buf = pubkey.to_bytes();
    Self(CompressedEdwardsY::from_slice(&buf).decompress().unwrap())
  }

  pub fn to_pubkey(&self) -> Pubkey {
    Pubkey::new_from_array(self.0.compress().to_bytes())
  }

  pub fn add(&self, other: Point) -> Point {
    Point(self.0.add(other.0))
  }

  pub fn sub(&self, other: Point) -> Point {
    Point(self.0.sub(other.0))
  }

  pub fn mul(&self, scalar: Scalar) -> Point {
    Point(self.0.mul(scalar))
  }
}
