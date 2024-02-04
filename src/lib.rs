//! A library to mimic The Evolution of Trust game by Nicky Case.
//!
//! This is made entirely by speculation and without reading the source code of the original
//! implementation.
//! There are some differences in compare to the original implemenation:
//! - The game matrices (the machine scores) can be asymmetrical.
//! - Different sandbox generation transfer algorithms (how winners should multiply).
//!
//! This crate has an optional "rand" feature which adds [`machines::MachineRandomizer`] and
//! [`players::Random`] which is disabled by default.
//!
//! To simulate a community, one needs a match ([`mod@matches`] or equal, ideally implementing
//! [`traits::MatchTrait`]), which is populated by players ([`players`] or equal, ideally
//! implementing [`traits::PlayerTrait`]).

pub(crate) mod worm_bools;

pub mod errors;
pub mod genetics;
pub mod machines;
pub mod matches;
pub mod matrices;
pub mod players;
pub mod traits;

/// Auto include traits.
pub mod prelude {
    pub use crate::traits::*;
}
