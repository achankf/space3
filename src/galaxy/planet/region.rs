use std::collections::HashSet;

use super::{
    environment::Environment, feature::Feature, habitat::Habitat, terrain::Terrain, Point,
};

pub type Id = u8;

pub struct RegionData {
    // graph
    kind: Terrain,
    neighbours: HashSet<Id>,
    coor: Point,

    /// immutable: max number of buildings in the habitat
    landmass: u8,
    habitat: Habitat,
    environment: Environment,
    modifiers: HashSet<Feature>,
}

pub struct Region {
    data: RegionData,
}

impl Region {
    pub fn new(data: RegionData) -> Self {
        Self { data }
    }
}
