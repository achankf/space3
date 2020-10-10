use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::building};

use super::character;

pub type Id = u16;

pub struct Dynasty {
    leader: character::Id,
    ownership: HashSet<building::Id>,
    prestige: u32,
}

pub struct Dynasties {
    dynasties: Entities<Dynasty, Id>,
    heirs: HashMap<Id, character::Id>,
}
