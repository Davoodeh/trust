//! Holds structs regarding payoff tables and such.

/// Holds the status on the game.
#[derive(Debug, Clone)]
pub struct GameMatrix<T> {
    /// The rewards for players both cooperating.
    pub cc: (T, T),
    /// The rewards for the first player cooperating and the other not.
    pub cd: (T, T),
    /// The rewards for the second player cooperating and the other not.
    pub dc: (T, T),
    /// The rewards for players both not cooperating.
    pub dd: (T, T),
}

impl Default for GameMatrix<isize> {
    fn default() -> Self {
        Self {
            cc: (2, 2),
            cd: (-1, 3),
            dc: (3, -1),
            dd: (0, 0),
        }
    }
}

impl<T> GameMatrix<T> {
    pub fn get_for_consents(&self, consents: (bool, bool)) -> &(T, T) {
        match consents {
            (true, true) => &self.cc,
            (true, false) => &self.cd,
            (false, true) => &self.dc,
            (false, false) => &self.dd,
        }
    }
}
