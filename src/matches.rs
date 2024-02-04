//! Different configurations for putting players against each other (1V1 and manyVmany).

use std::{marker::PhantomData, ops::AddAssign};

use crate::{
    errors::ArenaError,
    genetics::GeneticStrategy,
    machines::Machine,
    traits::{MachineTrait, MatchTrait, PlayerTrait},
};

/// A structure simulating two people playing a game.
#[derive(Debug)]
pub struct Match<T, P1, P2, M = Machine<T>> {
    /// The machine used in the match.
    pub machine: M,
    /// Players of the match.
    pub players: (P1, P2),
    pub phantom: PhantomData<T>,
}

impl<T, P1, P2, M> MatchTrait<T> for Match<T, P1, P2, M>
where
    T: AddAssign<T> + Clone + Default,
    P1: PlayerTrait<T>,
    P2: PlayerTrait<T>,
    M: MachineTrait<T>,
{
    fn play(&mut self) {
        let last_consents = (
            self.players.0.cooperation_consent(),
            self.players.1.cooperation_consent(),
        );
        let last_rewards = self.machine.play(last_consents).clone();

        // broadcast results to players
        self.players.1.memorize_last_game(
            (last_consents.1, last_consents.0),
            (last_rewards.1.clone(), last_rewards.0.clone()),
        );
        self.players
            .0
            .memorize_last_game(last_consents, last_rewards);
    }
}

impl<P1, P2> Default for Match<isize, P1, P2>
where
    P1: PlayerTrait<isize> + Default,
    P2: PlayerTrait<isize> + Default,
{
    fn default() -> Self {
        Self {
            machine: Default::default(),
            players: Default::default(),
            phantom: Default::default(),
        }
    }
}

/// A place where multiple opponents compete 2 by 2 and get removed and the best multiply.
pub struct Arena<T: Default + Clone, M = Machine<T>>
where
    T: Clone + Default,
    M: MachineTrait<T>,
{
    /// The rule of the base match for each 1v1 competition.
    machine: M,
    /// What type of players are present in the game (assumed forgotten version).
    player_constructors: Vec<Box<dyn PlayerTrait<T>>>,
    /// Players competing in the arena (holds the ID of `player_types`).
    players: Vec<usize>,
    /// What's every player's score.
    scores: Vec<T>,
    /// Rounds per play for each two opponents.
    rounds: usize,
    /// How to remove or multiply winners between each play (if needed).
    strategy: GeneticStrategy,
}

impl<T, M> Arena<T, M>
where
    T: Clone + Default + AddAssign<T>,
    M: MachineTrait<T>,
{
    /// Returns the arena or Err if players not in `0..player_constructors.len()`.
    pub fn new(
        machine: M,
        player_construtors: Vec<Box<dyn PlayerTrait<T>>>,
        players: Vec<usize>,
        rounds: usize,
        strategy: GeneticStrategy,
    ) -> Result<Self, ArenaError> {
        for &i in players.iter() {
            if i >= player_construtors.len() {
                return Err(ArenaError::UnknownPlayer);
            }
        }

        // make sure they are clean and forgotten everything in the past (to clone).
        let forgotten = player_construtors
            .into_iter()
            .map(|mut i| {
                i.forget_games();
                i
            })
            .collect();

        Ok(Self {
            player_constructors: forgotten,
            scores: Default::default(),
            strategy,
            machine,
            rounds,
            players,
        })
    }
}

impl<T, M> MatchTrait<T> for Arena<T, M>
where
    T: Clone + Default + AddAssign<T> + Ord,
    M: MachineTrait<T>,
{
    fn play(&mut self) {
        // reset scores.
        self.scores = vec![Default::default(); self.players.len()];

        for i in 0..self.players.len() {
            for j in (i + 1)..self.players.len() {
                // get both players cleared.
                let p1 = self.player_constructors[self.players[i]].clone();
                let p2 = self.player_constructors[self.players[j]].clone();

                // reset everything and make a match.
                self.machine.reset_scores();

                // play the rounds
                let ovo_results = {
                    let mut ovo = Match::<T, _, _, _> {
                        machine: &mut self.machine,
                        players: (p1, p2),
                        phantom: Default::default(),
                    };
                    for _ in 0..self.rounds {
                        ovo.play();
                    }
                    ovo.machine.scores()
                };

                // memorize the results
                self.scores[i] += ovo_results.0;
                self.scores[j] += ovo_results.1;
            }
        }

        // The best type of players (best at the end of the array).
        // TODO add other multiplication strategies for the next generation.
        let sorted_types = {
            let mut t = self
                .scores
                .clone()
                .into_iter()
                .enumerate()
                .map(|(t, v)| (self.players[t], v))
                .collect::<Vec<(usize, T)>>();
            t.sort_by_key(|(_, v)| v.clone());
            t.into_iter().map(|(t, _)| t).collect::<Vec<usize>>()
        };

        self.players = self.strategy.apply_to_vec(sorted_types);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::players::*;

    fn test_arena(
        copycats: (usize, isize),
        allcheaters: (usize, isize),
        allcooperates: (usize, isize),
        grudgers: (usize, isize),
        detectives: (usize, isize),
        kindcopycats: (usize, isize),
        simpletons: (usize, isize),
        next_gen_players: Vec<usize>,
    ) {
        let mut scores = vec![];

        let mut players: Vec<_> = vec![];
        for (i, c) in vec![
            copycats.0,
            allcheaters.0,
            allcooperates.0,
            grudgers.0,
            detectives.0,
            kindcopycats.0,
            simpletons.0,
        ]
        .into_iter()
        .enumerate()
        {
            players.append(&mut vec![i; c]);
        }

        let mut arena = Arena {
            machine: Machine::default(),
            rounds: 10,
            scores: vec![0; players.len()],
            player_constructors: vec![
                Box::new(CopyCat::default()),
                Box::new(AllCheat::default()),
                Box::new(AllCooperate::default()),
                Box::new(Grudger::default()),
                Box::new(Detective::default()),
                Box::new(KindCopyCat::default()),
                Box::new(Simpleton::default()),
            ],
            players,
            strategy: GeneticStrategy::CullingElitism(5, 5),
        };
        arena.play();

        scores.append(&mut vec![copycats.1; copycats.0]);
        scores.append(&mut vec![allcheaters.1; allcheaters.0]);
        scores.append(&mut vec![allcooperates.1; allcooperates.0]);
        scores.append(&mut vec![grudgers.1; grudgers.0]);
        scores.append(&mut vec![detectives.1; detectives.0]);
        scores.append(&mut vec![kindcopycats.1; kindcopycats.0]);
        scores.append(&mut vec![simpletons.1; simpletons.0]);

        assert_eq!(arena.scores, scores);

        arena.players.sort();
        assert_eq!(arena.players, next_gen_players)
    }

    #[test]
    fn test_arena_1_step() {
        test_arena(
            (25, 480),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        );
        test_arena(
            (24, 459),
            (1, 72),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        );
        test_arena(
            (1, -24),
            (24, 3),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
        );
        test_arena(
            (9, 312),
            (8, 267),
            (8, 240),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2,
            ],
        );
        test_arena(
            (13, 480),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (12, 480),
            (0, 0),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            ],
        );
        test_arena(
            (7, 249),
            (11, 63),
            (0, 0),
            (0, 0),
            (0, 0),
            (7, 238),
            (0, 0),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 5, 5, 5, 5, 5, 5, 5,
            ],
        );
        test_arena(
            (0, 0),
            (0, 0),
            (25, 480),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            ],
        );
        test_arena(
            (6, 390),
            (4, 207),
            (3, 297),
            (3, 357),
            (3, 288),
            (3, 341),
            (3, 353),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6,
            ],
        );
    }

    #[test]
    fn test_machine_default_allcheat_allcheat() {
        let mut game = Match::<isize, AllCheat, AllCheat>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (0, 0));
    }

    #[test]
    fn test_machine_default_allcooperate_allcooperate() {
        let mut game = Match::<isize, AllCooperate, AllCooperate>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (10, 10));
    }

    #[test]
    fn test_machine_default_copycat_copycat() {
        let mut game = Match::<isize, CopyCat, CopyCat>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (10, 10));
    }

    #[test]
    fn test_machine_default_copycat_allcooperate() {
        let mut game = Match::<isize, AllCooperate, CopyCat>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (10, 10));
    }

    #[test]
    fn test_machine_default_allcheat_allcooperate() {
        let mut game = Match::<isize, AllCheat, AllCooperate>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (15, -5));
    }

    #[test]
    fn test_machine_default_allcheat_copycat() {
        let mut game = Match::<isize, AllCheat, CopyCat>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (3, -1));
    }

    #[test]
    fn test_machine_default_allcheat_kindcopycat() {
        let mut game = Match::<isize, AllCheat, KindCopyCat>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (6, -2));
    }

    #[test]
    fn test_machine_default_allcheat_simpleton() {
        let mut game = Match::<isize, AllCheat, Simpleton>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (9, -3));
    }

    #[test]
    fn test_machine_default_allcooperate_simpleton() {
        let mut game = Match::<isize, AllCooperate, Simpleton>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (10, 10));
    }

    #[test]
    fn test_machine_default_allcooperate_detective() {
        let mut game = Match::<isize, AllCooperate, Detective>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (4, 12));
    }

    #[test]
    fn test_machine_default_allcheat_detective() {
        let mut game = Match::<isize, AllCheat, Detective>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (9, -3));
    }

    #[test]
    fn test_machine_default_copycat_detective() {
        let mut game = Match::<isize, CopyCat, Detective>::default();
        game.play_for_rounds(5);
        assert_eq!(game.machine.scores, (8, 8));
    }
}
