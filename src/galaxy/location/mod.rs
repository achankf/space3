use std::collections::HashMap;

use crate::galaxy::{
    living::character,
    planet::{self, region},
    Point,
};

use super::{group};

pub struct Locations {
    planet_region: HashMap<character::Id, (planet::Id, region::Id)>,
    planet_group: HashMap<group::Id, Point>,

    space_group: HashMap<group::Id, Point>,
}
