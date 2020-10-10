use std::collections::HashSet;

use crate::{entity::Entities, galaxy::living::character};

use super::Id;

pub struct Farm {
    // how much land can be farmed and taxed
    land: u8,

    // number of expansions of the farms silo storage
    silo: u8,

    // how much crops can be harvested in harvest season
    growth: u32,

    // storage
    output: u32,
    water: u32,
    fertilizer: u32,
    tools: u32,
    vehicle: u32,

    // embraced technologies & efficiency
    with_tools: Option<u32>,
    with_fertilizer: Option<u32>,
    with_vehicles: Option<u32>,
    with_vertical: Option<u32>,  // maximize yield per unit land
    with_hydoponic: Option<u32>, // lower consumption

    farmers: HashSet<character::Id>,
}

pub struct Farms {
    buildings: Entities<Farm, Id>,
}
