use std::sync::{LazyLock, Mutex};

use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::Score;

#[derive(Resource, Reflect, Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighScoreBoard {
    scores: Vec<HighScore>,
}

#[derive(
    Debug, Display, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Reflect,
)]
#[display("{}: {}", name, score)]
pub struct HighScore {
    pub score: Score,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct HighScoreKey(Uuid);

const DEFAULT_USER: &str = "New Player";
const NUM_HIGH_SCORES: usize = 10;

static SCORE_INDEX_KEYS: LazyLock<Mutex<HashMap<HighScoreKey, usize>>> =
    LazyLock::new(Mutex::default);

impl HighScoreBoard {
    pub fn iter(&self) -> impl Iterator<Item = &HighScore> {
        self.scores.iter()
    }

    pub fn add_score(&mut self, score: Score) -> Option<HighScoreKey> {
        let key = HighScoreKey(Uuid::now_v7());

        if let Some((index, _)) = self
            .scores
            .iter()
            .enumerate()
            .find(|(_idx, highscore)| score > highscore.score)
        {
            debug!(?score, ?index, "found new highscore");
            SCORE_INDEX_KEYS.lock().unwrap().insert(key, index);
            self.scores.insert(
                index,
                HighScore {
                    name: DEFAULT_USER.to_string(),
                    score,
                },
            );
            while self.scores.len() > NUM_HIGH_SCORES {
                self.scores.remove(NUM_HIGH_SCORES);
            }
            return Some(key);
        };

        if self.scores.len() < NUM_HIGH_SCORES {
            let index = self.scores.len();
            debug!(?score, ?index, "found new highscore");
            SCORE_INDEX_KEYS.lock().unwrap().insert(key, index);
            self.scores.push(HighScore {
                name: DEFAULT_USER.to_string(),
                score,
            });
            return Some(key);
        }

        None
    }

    pub fn assign_name(&mut self, name: &str, key: HighScoreKey) {
        let mut score_keys = SCORE_INDEX_KEYS.lock().unwrap();
        if let Some(index) = score_keys.remove(&key) {
            self.scores[index].name = name.to_string();
        }
    }
}
