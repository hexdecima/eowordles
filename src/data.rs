use eowordle_lib::prelude::{Enemy, EnemyDiff};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guess {
    pub enemy: Enemy,
    pub diff: EnemyDiff,
}
