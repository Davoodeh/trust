//! A series of deterministic players introduced in the original game.
//!
//! Enable "rand" feature for the player Random.

use crate::{traits::PlayerTrait, worm_bools::RiseOnlyBool};

/// Start with cooperating and repeat whatever the opponent does the last round.
#[derive(Debug, Default, Clone)]
pub struct CopyCat {
    last_enemy_consent: Option<bool>,
}

impl<T> PlayerTrait<T> for CopyCat {
    fn cooperation_consent(&self) -> bool {
        self.last_enemy_consent.unwrap_or(true)
    }

    fn memorize_last_game(&mut self, last_consents: (bool, bool), _last_rewards: (T, T)) {
        self.last_enemy_consent = Some(last_consents.1);
    }

    fn forget_games(&mut self) {
        *self = Default::default();
    }
}

/// A player who always cooperates.
#[derive(Debug, Default, Clone, Copy)]
pub struct AllCooperate;

impl<T> PlayerTrait<T> for AllCooperate {
    fn cooperation_consent(&self) -> bool {
        true
    }
}

/// A player who always cheats.
#[derive(Debug, Default, Clone, Copy)]
pub struct AllCheat;

impl<T> PlayerTrait<T> for AllCheat {
    fn cooperation_consent(&self) -> bool {
        false
    }
}

/// Cooperate till never been cheated.
#[derive(Debug, Default, Clone, Copy)]
pub struct Grudger {
    been_cheated: RiseOnlyBool,
}

impl<T> PlayerTrait<T> for Grudger {
    fn cooperation_consent(&self) -> bool {
        !self.been_cheated
    }

    fn memorize_last_game(&mut self, last_consents: (bool, bool), _last_rewards: (T, T)) {
        self.been_cheated.rise_if(!last_consents.1);
    }

    fn forget_games(&mut self) {
        *self = Self::default();
    }
}

/// Plays a fixed strategy and if cheated turns to copycat else cheats.
#[derive(Debug, Clone)]
pub struct Detective {
    been_cheated_in_analysing: RiseOnlyBool,
    /// How far the detective is in the analysis.
    analysing_stage: usize,
    /// Determines how the detective analysis the user.
    analysing_strategy: Vec<bool>,
    /// The memory of what to play next.
    next_strategy: bool,
}

impl Detective {
    pub fn new(initial_strategy: Vec<bool>) -> Self {
        Self {
            next_strategy: *initial_strategy.first().unwrap_or(&true),
            analysing_strategy: initial_strategy,
            analysing_stage: 1,
            been_cheated_in_analysing: Default::default(),
        }
    }
}

impl Default for Detective {
    fn default() -> Self {
        Self::new(vec![true, false, true, true])
    }
}

impl<T> PlayerTrait<T> for Detective {
    fn cooperation_consent(&self) -> bool {
        self.next_strategy
    }

    fn memorize_last_game(&mut self, last_consents: (bool, bool), _last_rewards: (T, T)) {
        // Return the current step of the analyses and count one upward (None if ended).
        if self.analysing_stage < self.analysing_strategy.len() {
            // memorize if enemy did retaliate ever
            self.been_cheated_in_analysing.rise_if(!last_consents.1);
            self.next_strategy = self.analysing_strategy[self.analysing_stage];
            self.analysing_stage += 1; // do not increase post-analysis not to overflow hence in if
        } else {
            self.next_strategy = *self.been_cheated_in_analysing && last_consents.1;
        }
    }

    fn forget_games(&mut self) {
        self.next_strategy = *self.analysing_strategy.first().unwrap_or(&true);
        self.analysing_stage = 1;
        self.been_cheated_in_analysing = Default::default();
    }
}

/// Copy kitten, allows for a number of repeated cheats before retaliating.
#[derive(Debug, Clone, Copy)]
pub struct KindCopyCat {
    /// Only retaliates after the number of mistakes is passed.
    mistakes_allowed: usize,
    /// How many times has been cheated in row.
    cheated_in_row: usize,
}

impl KindCopyCat {
    pub fn new(mistakes_allowed: usize) -> Self {
        Self {
            mistakes_allowed,
            cheated_in_row: 0,
        }
    }
}

impl Default for KindCopyCat {
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T> PlayerTrait<T> for KindCopyCat {
    fn cooperation_consent(&self) -> bool {
        self.cheated_in_row <= self.mistakes_allowed
    }

    fn memorize_last_game(&mut self, last_consents: (bool, bool), _last_rewards: (T, T)) {
        if last_consents.1 {
            self.cheated_in_row = 0;
        } else {
            // if already distrustful (no consent), do not increase the counter not to overflow
            if <Self as PlayerTrait<T>>::cooperation_consent(self) {
                self.cheated_in_row += 1;
            }
        }
    }

    fn forget_games(&mut self) {
        self.cheated_in_row = Default::default();
    }
}

/// Start by cooperate and if cooperated, repeats last move else, does opposite of the last.
///
/// Note that this player sees what are the results of the machine. Since machines and arenas may
/// not be fair, he repeats the last thing, even if it was a mistake.
#[derive(Debug, Clone, Copy)]
pub struct Simpleton {
    next_move: bool,
}

impl Default for Simpleton {
    fn default() -> Self {
        Self { next_move: true }
    }
}

impl<T> PlayerTrait<T> for Simpleton {
    fn cooperation_consent(&self) -> bool {
        self.next_move
    }

    fn memorize_last_game(&mut self, last_consents: (bool, bool), _last_rewards: (T, T)) {
        self.next_move = if last_consents.1 {
            last_consents.0
        } else {
            !last_consents.0
        };
    }

    fn forget_games(&mut self) {
        *self = Default::default();
    }
}

/// Randomly consents or doesn't (requires "rand" feature).
#[cfg(any(feature = "rand", doc))]
#[derive(Default, Debug, Clone, Copy)]
pub struct Random;

#[cfg(any(feature = "rand", doc))]
impl<T> PlayerTrait<T> for Random {
    fn cooperation_consent(&self) -> bool {
        rand::random()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_types<P: PlayerTrait<usize>>(
        player: &mut P,
        initial: bool,
        enemy_consents_to_reaction: &[((bool, bool), bool)],
    ) {
        assert_eq!(player.cooperation_consent(), initial); // first must be true

        for &(consents, reaction) in enemy_consents_to_reaction {
            player.memorize_last_game(consents, (1, 1));
            assert_eq!(player.cooperation_consent(), reaction);
        }
    }

    #[test]
    fn test_copycat() {
        all_types(
            &mut CopyCat::default(),
            true,
            &[
                ((true, true), true),
                ((true, false), false),
                ((false, false), false),
                ((false, true), true),
            ],
        );
    }

    #[test]
    fn test_all_cooperate() {
        all_types(
            &mut AllCooperate::default(),
            true,
            &[
                ((true, true), true),
                ((true, false), true),
                ((false, false), true),
                ((false, true), true),
            ],
        );
    }

    #[test]
    fn test_all_cheat() {
        all_types(
            &mut AllCheat::default(),
            false,
            &[
                ((true, true), false),
                ((true, false), false),
                ((false, false), false),
                ((false, true), false),
            ],
        );
    }

    #[test]
    fn test_grudger() {
        all_types(
            &mut Grudger::default(),
            true,
            &[
                ((true, true), true),
                ((true, false), false),
                ((false, true), false),
                ((true, true), false),
            ],
        );

        all_types(
            &mut Grudger::default(),
            true,
            &[
                ((true, true), true),
                ((true, true), true),
                ((false, true), true),
                ((true, true), true),
                ((true, false), false),
                ((true, true), false),
                ((true, true), false),
            ],
        );
    }

    #[test]
    fn test_detective() {
        all_types(
            &mut Detective::default(),
            true,
            &[
                // default initial strat
                ((false, true), false),
                ((false, false), true),
                ((false, true), true),
                // copycat, coz been cheated
                ((false, false), false),
                ((false, false), false),
                ((false, false), false),
                ((false, true), true),
                ((false, true), true),
                ((false, false), false),
                ((true, false), false),
                ((true, true), true),
            ],
        );

        all_types(
            &mut Detective::default(),
            true,
            &[
                // default initial strat
                ((false, true), false),
                ((false, true), true),
                ((false, true), true),
                // cheater, coz not been cheated
                ((false, true), false),
                ((false, true), false),
                ((false, false), false),
                ((false, true), false),
                ((false, true), false),
                ((false, false), false),
                ((true, false), false),
                ((true, true), false),
            ],
        );
    }

    #[test]
    fn test_kindcopycat() {
        all_types(
            &mut KindCopyCat::default(),
            true,
            &[
                ((false, true), true),
                ((true, true), true),
                ((true, false), true),
                ((false, true), true),
                ((true, false), true),
                ((false, true), true),
                ((true, false), true),
                ((false, false), false),
                ((false, true), true),
                ((false, false), true),
                ((true, false), false),
                ((true, false), false),
                ((false, true), true),
            ],
        );
    }

    #[test]
    fn test_simpleton() {
        all_types(
            &mut Simpleton::default(),
            true,
            &[
                ((true, true), true),
                ((true, true), true),
                ((true, false), false),
                ((false, true), false),
                ((false, false), true),
                ((true, false), false),
                ((false, true), false),
            ],
        );
    }
}
