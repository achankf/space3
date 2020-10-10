use std::collections::HashSet;

use crate::{entity::Entities, galaxy::building};

use super::character;

pub type Id = u16;

pub struct Dynasty {
    leader: character::Id,
    ownership: HashSet<building::Id>,
    prestige: u32,
    heir: Option<Id>,
}

pub struct Dynasties(Entities<Dynasty, Id>);
