use std::collections::HashSet;

use crate::entity::{Tec};

use super::living::character;

pub type Id = u32;

pub struct Group {
    members: HashSet<character::Id>,
}

pub struct Groups {
    groups: Tec<Group, Id>,
}
