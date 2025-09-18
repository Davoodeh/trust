//! Helpers regarding genetic manipulation and evolution algorithms.

/// Strategies regarding moving from one generation to another in genetic settings.
pub enum GeneticStrategy {
    /// Keep the population as given (no change).
    Keep,
    /// Remove the worst and multiply the best.
    CullingElitism(usize, usize),
}

impl GeneticStrategy {
    /// Apply a strategy on the scores acquired by each type (usize is the ID/type of group).
    ///
    /// This only works if the list is already sorted from the best type to the worst in score.
    /// Note that the results may be unsorted.
    pub fn apply_to_vec(&self, mut sorted_types: Vec<usize>) -> Vec<usize> {
        if sorted_types.is_empty() {
            return vec![];
        }

        match self {
            Self::Keep => {}
            Self::CullingElitism(to_remove, to_add) => {
                let best = *sorted_types.last().unwrap();

                // remove only if possible
                let to_remove = if sorted_types.len() <= *to_remove {
                    sorted_types.len()
                } else {
                    *to_remove
                };

                for i in 0..to_remove {
                    sorted_types.swap_remove(i);
                }

                for _ in 0..*to_add {
                    sorted_types.push(best);
                }
            }
        }

        sorted_types
    }
}
