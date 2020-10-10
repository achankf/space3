use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::squad;

pub type Id = u32;

pub struct Platoon {
    squads: HashSet<squad::Id>,
}

pub struct Platoons {
    leaders: HashMap<Id, character::Id>,
    platoon: Entities<Platoon, Id>,
}
