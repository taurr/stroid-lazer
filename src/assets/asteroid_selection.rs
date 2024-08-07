use bevy::prelude::*;
use bevy_turborand::RngComponent;
use serde::Deserialize;

use crate::utils::RngComponentExt;

//use super::optional;
use super::{AsteroidPool, AsteroidPoolCollection};

#[derive(Deserialize, Debug, Reflect, Clone)]
pub enum AsteroidSelection {
    Pool {
        key: String,
        #[serde(default = "AsteroidSelection::default_weight")]
        weight: f32,
    },
}

impl AsteroidSelection {
    fn default_weight() -> f32 {
        1.0
    }
}

pub trait AsteroidSplitSelectionExt {
    fn pick_random_pool<'a>(
        &self,
        rand: &mut RngComponent,
        pool_collection: &'a AsteroidPoolCollection,
    ) -> Option<&'a AsteroidPool>;
}

impl<C: AsRef<[AsteroidSelection]>> AsteroidSplitSelectionExt for C {
    fn pick_random_pool<'a>(
        &self,
        rand: &mut RngComponent,
        pool_collection: &'a AsteroidPoolCollection,
    ) -> Option<&'a AsteroidPool> {
        let slice = self.as_ref();

        let weight_sum = slice
            .iter()
            .map(|s| match s {
                AsteroidSelection::Pool { weight, .. } => *weight,
            })
            .sum();
        let random = rand.f32_range(0.0..weight_sum);

        let selection = {
            let mut w = 0.0;
            slice
                .iter()
                .find(|s| match s {
                    AsteroidSelection::Pool { weight, .. } => {
                        w += weight;
                        random < w
                    }
                })
                .unwrap()
        };

        match selection {
            AsteroidSelection::Pool { key: name, .. } => pool_collection.get(name),
        }
    }
}
