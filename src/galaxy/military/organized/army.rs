use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::corps;

pub type Id = u32;

pub struct Army {
    corps: HashSet<corps::Id>,
}

pub struct Armies {
    leaders: HashMap<Id, character::Id>,
    armies: Entities<Army, Id>,
}
