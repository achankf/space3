use std::collections::HashSet;

use crate::{
    entity::Entities,
    galaxy::{building, corporation, living::character},
};

pub type Id = u8;

pub enum Ownership {
    /// one person owns the entire company
    Proprietor(character::Id),
    /// this company is a fully-owned branch of a corporation
    Branch(corporation::Id),
}

pub struct OwnedBuildings {
    farms: HashSet<building::Id>,
}

pub struct Company {
    ownership: Ownership,
    owned_buildings: OwnedBuildings,
}

pub struct Companies {
    companies: Entities<Company, Id>,
}
