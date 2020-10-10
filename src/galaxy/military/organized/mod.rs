pub mod army;
pub mod company;
pub mod corps;
pub mod division;
pub mod platoon;
pub mod squad;

use std::collections::HashMap;

use crate::galaxy::{living::character, Date};

use self::{
    army::Armies, company::Companies, corps::Corpses, division::Divisions, platoon::Platoons,
    squad::Squads,
};

// https://www.army.mil/
// https://www.britannica.com/topic/military-unit
#[derive(PartialEq, PartialOrd)]
pub enum Rank {
    Private,    // troop - 1
    Sergeant,   // squad - 3
    Lieutenant, // platoon - 9
    Captain,    // company - 27

    // don't fight in direct combat
    Colonel, // division - 81
    General, // corps - 243

    // can only be promoted during wartime
    Marshal, // army - 729
}

pub struct Personnel {
    rank: Rank,
    battlefield_merits: u32,
    enlistment_date: Date,
}

pub struct Organized {
    personnel: HashMap<character::Id, Personnel>,
    squads: Squads,
    platoons: Platoons,
    companies: Companies,
    divisions: Divisions,
    corps: Corpses,
    armies: Armies,
}
