use std::collections::{HashMap, HashSet};

use crate::entity::Entities;

use super::{building, living::character};

pub type Id = u16;

pub struct FamilyStandings {}

pub struct Dynasty {
    members: HashMap<character::Id, FamilyStandings>,
    leader: character::Id,
    ownership: HashSet<building::Id>,
    prestige: u32,
}

pub struct Dynasties {
    dynasties: Entities<Dynasty, Id>,
    parent_dynasties: HashMap<Id, Id>,
}
