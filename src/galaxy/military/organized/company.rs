use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::platoon::{self};

pub type Id = u32;

pub struct Company {
    platoons: HashSet<platoon::Id>,
}

pub struct Companies {
    leaders: HashMap<Id, character::Id>,
    companies: Entities<Company, Id>,
}
