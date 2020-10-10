use std::collections::{HashMap, HashSet};

use crate::{entity::Entities, galaxy::living::character};

use super::Id;

pub struct Workshop {
    ownership: Id,
    employees: HashSet<character::Id>,
}

pub struct Workshops {
    buildings: Entities<Workshop, Id>,
    managers: HashMap<Id, character::Id>,
}
