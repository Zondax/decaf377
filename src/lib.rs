//! `decaf377` [instantiates Decaf over the BLS12-377 scalar
//! field](https://penumbra.zone/crypto/primitives/decaf377.html).
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate no_std_compat as std;

mod constants;
#[cfg(feature = "std")]
mod element;
#[cfg(feature = "std")]
mod elligator;
mod encoding;
mod error;
mod field_ext;
#[cfg(feature = "std")]
mod invsqrt;
mod on_curve;
mod ops;
#[cfg(feature = "std")]
pub mod rand;
pub mod serialize;
mod sign;

pub use constants::ZETA;
#[cfg(feature = "std")]
pub use element::{AffineElement, Element};
#[cfg(feature = "std")]
pub(crate) use element::{Decaf377EdwardsConfig, EdwardsProjective};
pub use encoding::Encoding;
pub use error::EncodingError;
pub use field_ext::FieldExt;

#[cfg(feature = "r1cs")]
pub mod r1cs;

pub use ark_bls12_377::Bls12_377;
pub use ark_ed_on_bls12_377::{Fq, Fr};

#[cfg(feature = "std")]
pub use invsqrt::SqrtRatioZeta;

use on_curve::OnCurve;
use sign::Sign;

#[cfg(feature = "std")]
/// Return the conventional generator for `decaf377`.
pub fn basepoint() -> Element {
    Element {
        inner: EdwardsProjective::new(
            *constants::B_X,
            *constants::B_Y,
            *constants::B_T,
            *constants::B_Z,
        ),
    }
}
