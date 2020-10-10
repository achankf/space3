use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::company;

pub type Id = u32;

pub struct Division {
    companies: HashSet<company::Id>,
}

pub struct Divisions {
    leaders: HashMap<Id, character::Id>,
    divisions: Entities<Division, Id>,
}
