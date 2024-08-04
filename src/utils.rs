use bevy_turborand::{DelegatedRng, RngComponent};
use std::ops::RangeBounds;

pub trait RngComponentExt {
    fn f32_range(&mut self, range: impl RangeBounds<f32>) -> f32;
}

impl RngComponentExt for RngComponent {
    fn f32_range(&mut self, range: impl RangeBounds<f32>) -> f32 {
        let start = match range.start_bound() {
            std::ops::Bound::Included(v) => *v,
            std::ops::Bound::Excluded(v) => *v + f32::EPSILON,
            std::ops::Bound::Unbounded => 0.0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(v) => *v,
            std::ops::Bound::Excluded(v) => *v - f32::EPSILON,
            std::ops::Bound::Unbounded => 1.0,
        };
        self.f32() * (end - start) + start
    }
}
