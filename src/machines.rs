//! Holds basic machines determining how games are played and how scores are recorded.
//!
//! Machines may malfunction randomly hence players must pay attention to the results registered by
//! the machines and base their decisions on that, not what they assume they played. This is the
//! reason a player type like [`crate::players::Simpleton`] sees the results returned by the
//! machine instead of what they put in the machine. In other words, a player must take action on
//! their registered state, not the state they assume they are in.

use std::ops::AddAssign;

use crate::{matrices::GameMatrix, traits::MachineTrait};

/// The main "engine" of the game which handles payoffs and costs.
///
/// This is a deterministic machine which works always according to the given matrix. This is the
/// default for most of the logic.
#[derive(Debug, Clone)]
pub struct Machine<T> {
    /// The game matrix regarding this machine.
    pub matrix: GameMatrix<T>,
    /// What are the current scores of this machine being played this much.
    pub scores: (T, T),
}

impl<T: Default> Machine<T> {
    pub fn new(matrix: GameMatrix<T>) -> Self {
        Self {
            matrix,
            scores: Default::default(),
        }
    }
}

impl Default for Machine<isize> {
    fn default() -> Self {
        Self {
            matrix: Default::default(),
            scores: Default::default(),
        }
    }
}

impl<T: Clone + Default + AddAssign<T>> MachineTrait<T> for Machine<T> {
    fn play_off_record(&self, consents: (bool, bool)) -> (T, T) {
        self.matrix.get_for_consents(consents).clone()
    }

    fn scores(&self) -> (T, T) {
        self.scores.clone()
    }

    fn reset_scores(&mut self) {
        self.scores = Default::default()
    }

    fn record_scores(&mut self, last_rewards: (T, T)) {
        self.scores.0 += last_rewards.0;
        self.scores.1 += last_rewards.1;
    }
}

/// A machine with chances of failure or swapping outputs (requires feature "rand").
#[cfg(any(feature = "rand", doc))]
pub struct MachineRandomizer<T> {
    pub base: Machine<T>,
    /// What are the chances that the player will convert their positive consent to false (`0..=1`).
    pub consent_falsify_chance: (f32, f32),
    /// What are the chances that the player will convert their negative consent to true (`0..=1`).
    pub random_consenter: (f32, f32),
}

#[cfg(any(feature = "rand", doc))]
impl<T: Clone + Default + AddAssign<T>> MachineTrait<T> for MachineRandomizer<T> {
    fn play_off_record(&self, mut consents: (bool, bool)) -> (T, T) {
        // mutate the contests randomly.
        let mut rng = rand::thread_rng();
        let chances = (
            <rand::rngs::ThreadRng as rand::Rng>::gen::<f32>(&mut rng),
            <rand::rngs::ThreadRng as rand::Rng>::gen::<f32>(&mut rng),
        );

        if consents.0 {
            consents.0 = chances.0 > self.consent_falsify_chance.0;
        } else {
            consents.0 = chances.0 <= self.random_consenter.0;
        }

        if consents.1 {
            consents.1 = chances.1 > self.consent_falsify_chance.1;
        } else {
            consents.1 = chances.1 <= self.random_consenter.1;
        }

        self.base.play_off_record(consents)
    }

    fn scores(&self) -> (T, T) {
        self.base.scores()
    }

    fn reset_scores(&mut self) {
        self.base.reset_scores()
    }

    fn record_scores(&mut self, last_rewards: (T, T)) {
        self.base.record_scores(last_rewards)
    }
}
