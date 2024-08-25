use std::sync::{LazyLock, Mutex};

use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::Score;

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct HighScoreBoard {
    scores: Vec<HighScore>,
}

#[derive(Debug, Display, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[display("{name}: {score}")]
pub struct HighScore {
    score: Score,
    name: String,
    datetime: std::time::SystemTime,
}

impl HighScore {
    pub fn new(name: impl Into<String>, score: Score) -> Self {
        Self {
            name: name.into(),
            score,
            datetime: std::time::SystemTime::now(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn datetime(&self) -> std::time::SystemTime {
        self.datetime
    }
}

#[derive(Resource, Debug, Eq)]
pub struct HighScoreKey {
    key: Option<Uuid>,
    place: usize,
}

impl HighScoreKey {
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

static SCORE_INDEX_KEYS: LazyLock<Mutex<HashMap<Uuid, usize>>> = LazyLock::new(Mutex::default);

impl HighScoreBoard {
    pub fn iter(&self) -> impl Iterator<Item = &HighScore> {
        self.scores.iter()
    }

    pub fn add_score(
        &mut self,
        score: Score,
        old_key: Option<&HighScoreKey>,
    ) -> Option<HighScoreKey> {
        let mut place_map = SCORE_INDEX_KEYS.lock().unwrap();

        if let Some(HighScoreKey { key: Some(key), .. }) = old_key {
            if let Some(old_place) = place_map.remove(key) {
                self.scores.remove(old_place);
            }
        }

        if let Some((place, _)) = self
            .scores
            .iter()
            .enumerate()
            .find(|(_idx, highscore)| score > highscore.score)
        {
            debug!(?score, ?place, "found new highscore");

            let key = Uuid::now_v7();
            self.scores
                .insert(place, HighScore::new(DEFAULT_USER, score));
            place_map.insert(key, place);

            while self.scores.len() > NUM_HIGH_SCORES {
                self.scores.remove(NUM_HIGH_SCORES);
            }

            return Some(HighScoreKey {
                key: Some(key),
                place,
            });
        };

        if self.scores.len() < NUM_HIGH_SCORES {
            let place = self.scores.len();
            debug!(?score, ?place, "found new highscore");

            let key = Uuid::now_v7();
            self.scores.push(HighScore::new(DEFAULT_USER, score));
            place_map.insert(key, place);

            return Some(HighScoreKey {
                key: Some(key),
                place,
            });
        }

        None
    }

    pub fn assign_name(&mut self, name: &str, key: &mut HighScoreKey) {
        let Some(key) = key.key.take() else {
            return;
        };
        let mut place_map = SCORE_INDEX_KEYS.lock().unwrap();
        if let Some(index) = place_map.remove(&key) {
            self.scores[index].name = name.to_string();
        }
    }
}
