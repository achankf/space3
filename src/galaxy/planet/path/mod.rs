use std::collections::HashMap;

use crate::galaxy::living::character::{self, Character};

use super::region;

pub mod defense;
pub mod platform;

struct Path(region::Id, region::Id);

impl Path {
    pub fn new(i: region::Id, j: region::Id) -> Self {
        if i < j {
            Self(i, j)
        } else {
            Self(j, i)
        }
    }
}

pub struct Defense {
    watchtower: u32,
}

pub struct Data {
    /// decides what kind of path this is (i.e. paved roads, railroad, etc.)
    level: u8,

    /// how well the route is maintained
    condition: u32,
}

impl Data {
    pub fn max_speed(&self) -> f32 {
        todo!()
    }

    pub fn best_condition(&self) -> f32 {
        todo!()
    }

    pub fn power_usage(&self) -> f32 {
        todo!()
    }
}

pub struct Paths(HashMap<Path, Data>);
