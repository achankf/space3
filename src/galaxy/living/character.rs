use enum_map::{Enum, EnumMap};
use std::collections::HashMap;

use crate::entity::Entities;

pub type Id = u32;

pub enum Experience {
    // common
    Labour,      // all types of manual labour
    Social,      // knowing about society, dealing with relationships
    Math,        // the more you know, the more you can achieve, given enough resources
    Empathy,     // ability to see others' stats
    Science,     // knowledge in science
    Cultivation, // martial arts
    Mysticism,   // religion
}

#[derive(Enum)]
pub enum Race {
    Dwarf,  //
    Elf,    // create parts from woods as much as from plates
    Globin, //
    Human,  // devolved human
    Merman, // can settle in Ocean regions, high intelligence
    Orc,    //

    // aliens
    Izzaz, // insect race, hive-mind, one family and one queen per region, explosive growth rate, drastic difference in stats in the hierarchy
    Pneit, // bird race, ferocious warriors, miliary-focus society who'd wish to enslave everyone, typical "bad guys", bird brain
    Yuig,  // crystalloid race, primarily feed on energy
}

pub enum Sex {
    Male,
    Female,
    Futanari, // have both sexes, need 2 to reproduce
    Asexual,  // reproduce by itself
    Machine,  // produced in factories
}

pub struct Character {
    strength: u8,
    constituion: u8,
    dexterity: u8,
    agility: u8,
    intelligence: u8,
    charisma: u8,
    sense: u8,   // use of external energy
    control: u8, // use of internal energy
    luck: u8,

    experiences: HashMap<Experience, u32>,

    // life of the character
    life_energy: u32,
    born: u32,

    race: EnumMap<Race, f32>,
    sex: Sex,

    parent: Parents,
}

pub enum Parents {
    None,
    One(Id),
    Two(Id, Id),
}

pub struct Characters {
    entities: Entities<Character, Id>,
}

pub struct KinshipResult {
    /// lowest common ancestor
    lca: Id,

    // distance from LCA
    target_distance: u32,
    other_distance: u32,
}

impl Characters {
    pub fn kinship(&self, _target: Id, _other_character: Id) -> Option<KinshipResult> {
        todo!()
    }
}
