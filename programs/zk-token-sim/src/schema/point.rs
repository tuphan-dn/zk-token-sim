use anchor_lang::prelude::*;
use borsh::{
  maybestd::io::{Result, Write},
  BorshDeserialize, BorshSerialize,
};
use core::ops::{Add, Mul, Sub};
use curve25519_dalek::{
  constants::{ED25519_BASEPOINT_COMPRESSED, ED25519_BASEPOINT_POINT},
  edwards::{CompressedEdwardsY, EdwardsPoint},
  scalar::Scalar,
};
use lazy_static::lazy_static;
use sha3::Sha3_512;

lazy_static! {
  /// Pedersen base point for encoding messages to be committed.
  pub static ref G: EdwardsPoint = ED25519_BASEPOINT_POINT;
  /// Pedersen base point for encoding the commitment openings.
  pub static ref H: EdwardsPoint = EdwardsPoint::hash_from_bytes::<Sha3_512>(ED25519_BASEPOINT_COMPRESSED.as_bytes());
}

/**
 * EC Point
 */
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point {
  value: EdwardsPoint,
}

impl BorshSerialize for Point {
  fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
    let bytes = self.value.compress().to_bytes();
    msg!("serialize {}", bytes.len());
    writer.write_all(&bytes)
  }
}

impl BorshDeserialize for Point {
  fn deserialize(buf: &mut &[u8]) -> Result<Self> {
    msg!("deserialize: buf.len {}", buf.len());

    let compressed_point = Box::new(CompressedEdwardsY::from_slice(buf));
    msg!("compressed_point {:?}", compressed_point);
    msg!("point {:?}", compressed_point.decompress());

    let point = Point {
      value: compressed_point.decompress().unwrap(),
    };

    Ok(point)
  }
}

impl Point {
  // pub const G: EdwardsPoint = ED25519_BASEPOINT_POINT;

  pub fn add(&self, other: Point) -> Point {
    let p = self.value.add(other.value);
    return Point { value: p };
  }

  pub fn sub(&self, other: Point) -> Point {
    let p = self.value.sub(other.value);
    return Point { value: p };
  }

  pub fn mul(&self, scalar: Scalar) -> Point {
    let p = self.value.mul(scalar);
    return Point { value: p };
  }
}
