use std::collections::HashSet;

use self::{environment::Environment, feature::Feature, habitat::Habitat, terrain::Terrain};

use super::Point;

pub mod company;
pub mod council;
pub mod dungeon;
pub mod environment;
pub mod feature;
pub mod habitat;
pub mod terrain;

pub type Id = u8;

pub struct Region {
    kind: Terrain,
    /// immutable: max number of buildings in the habitat
    landmass: u8,
    habitat: Habitat,
    environment: Environment,
    modifiers: HashSet<Feature>,
    coor: Point,
}
