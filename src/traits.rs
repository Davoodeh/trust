//! Holds all the traits for this crate.

use auto_impl::auto_impl;
use dyn_clone::DynClone;

/// Determines the behaviour of the player.
///
/// Provided [`Self::memorize_last_game`] and [`Self::forget_games`] are implemented, a match
/// coordinator ([`MatchTrait`] implementor ideally) must call these methods when appropriate for
/// the players to know how to play against each other. In a more analogous way, players must know
/// when each round starts and ends. If not told, they cannot decide rationally so the coordinators
/// must inform the players accordingly.
///
/// Some players, however, lack memory and always (i.e. [`crate::players::AllCooperate`]) play a
/// preset strategy. Those do not require the methods.
#[auto_impl(&mut, Box)]
pub trait PlayerTrait<T>: DynClone {
    /// Determine whether the player should cooperate or not (player's answer to the next round).
    fn cooperation_consent(&self) -> bool;

    /// Add the last game to the memory to make observations based on that.
    #[allow(unused_variables)]
    fn memorize_last_game(&mut self, last_consents: (bool, bool), last_rewards: (T, T)) {}

    /// Reset the memory.
    fn forget_games(&mut self) {}
}

impl<T> Clone for Box<dyn PlayerTrait<T>>
where
    dyn PlayerTrait<T>: DynClone,
{
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

/// A machine receiving inputs from players and putting out the results.
#[auto_impl(&mut, Box)]
pub trait MachineTrait<T: Clone> {
    /// Play a game and return the results (do not record the results anywhere).
    fn play_off_record(&self, consents: (bool, bool)) -> (T, T);

    /// Get the scores.
    fn scores(&self) -> (T, T);

    /// Reset the machine stats.
    fn reset_scores(&mut self);

    /// Add another round of scores the leaderboard.
    fn record_scores(&mut self, last_rewards: (T, T));

    /// Play the inputs and get the outputs (mutating scoreboard and recording each result).
    fn play(&mut self, consents: (bool, bool)) -> (T, T) {
        let last_rewards = self.play_off_record(consents);
        self.record_scores(last_rewards.clone());
        last_rewards
    }
}

/// A match for two players (consecutive plays on a machine).
pub trait MatchTrait<T> {
    /// Play the next round and save it in the machine.
    fn play(&mut self);

    /// Play the number of rounds in succession.
    fn play_for_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.play();
        }
    }
}
