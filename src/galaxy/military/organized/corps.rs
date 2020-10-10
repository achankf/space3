use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::division;

pub type Id = u32;

pub struct Corps {
    divisions: HashSet<division::Id>,
}

// need a plural for Corps
pub struct Corpses {
    leaders: HashMap<Id, character::Id>,
    corps: Entities<Corps, Id>,
}
