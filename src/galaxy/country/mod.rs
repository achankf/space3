use std::collections::{HashMap, HashSet};

use super::planet::{self, region};

pub type Id = u32;

pub struct Regions {
    controlled: HashMap<planet::Id, HashSet<region::Id>>,
    culture: HashMap<planet::Id, HashMap<region::Id, u32>>,
}

pub struct Country {
    regions: Regions,
}
