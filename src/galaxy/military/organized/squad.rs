use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

pub type Id = u32;

pub struct Squad {
    sergeant: character::Id,
    troops: HashSet<character::Id>,
}

pub struct Squads {
    leaders: HashMap<Id, character::Id>,
    squads: Entities<Squad, Id>,
}
