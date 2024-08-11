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

#[derive(Component, Debug, Eq, Clone, Copy)]
pub struct HighScoreKey {
    place: usize,
    key: Uuid,
}

impl HighScoreKey {
    pub fn new() -> Self {
        Self {
            key: Uuid::now_v7(),
            place: usize::MAX,
        }
    }

    pub fn with_place(self, place: usize) -> Self {
        Self { place, ..self }
    }

    pub fn place(&self) -> usize {
        self.place
    }
}

impl std::hash::Hash for HighScoreKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl PartialEq for HighScoreKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

const DEFAULT_USER: &str = "New Player";
const NUM_HIGH_SCORES: usize = 10;

static SCORE_INDEX_KEYS: LazyLock<Mutex<HashMap<HighScoreKey, usize>>> =
    LazyLock::new(Mutex::default);

impl HighScoreBoard {
    pub fn iter(&self) -> impl Iterator<Item = &HighScore> {
        self.scores.iter()
    }

    pub fn add_score(
        &mut self,
        score: Score,
        old_key: Option<&HighScoreKey>,
    ) -> Option<HighScoreKey> {
        let key = old_key
            .cloned()
            .inspect(|k| {
                SCORE_INDEX_KEYS.lock().unwrap().remove(k);
            })
            .unwrap_or_else(HighScoreKey::new);

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
            return Some(key.with_place(index));
        };

        if self.scores.len() < NUM_HIGH_SCORES {
            let index = self.scores.len();
            debug!(?score, ?index, "found new highscore");
            SCORE_INDEX_KEYS.lock().unwrap().insert(key, index);
            self.scores.push(HighScore {
                name: DEFAULT_USER.to_string(),
                score,
            });
            return Some(key.with_place(index));
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
