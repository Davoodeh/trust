//! Holds all the errors in this crate.

use std::fmt;

/// Indicates a failure in [`crate::matches::Arena`].
#[derive(Debug)]
pub enum ArenaError {
    /// Thrown when a player type cannot be known (ID larger than types).
    UnknownPlayer,
}

impl fmt::Display for ArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownPlayer => "The given ID in the players list is not in constructors.",
            }
        )
    }
}

impl std::error::Error for ArenaError {}
