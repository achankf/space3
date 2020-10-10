use std::collections::{HashMap, HashSet};

use super::{building, living::character, Date};

pub type SpecializationId = u32;
pub type FactionId = u32;

pub enum FactionKind {
    /// - controls the private sector (industrial, financial, service)
    /// - maximize profits for shareholders
    Economic,

    /// - generate more political power naturally
    /// - research political theories to advance better forms of government
    /// - make decisions for its country
    Politics,

    /// - controls the military
    /// - lean towards autocracy
    /// - can be armies, mercenaries, martial art sects, or criminals
    Military,
}

pub enum Term {
    Y1,
    Y2,
    Y4,
    Y10,
    Y20,
    Y50,
    Y100,
    Y400,
    Y1000,
    ForLife,
}

pub enum Designation {
    /// the party revolve around 1 person, and it disbands after the leader dies.
    /// For example, any royalty can form this kind of party for the purpose of succession.
    Personal,

    /// the leader appoints an heir in this party; if no heir, do an election
    Inherit(Option<character::Id>),

    /// all members give a vote and the one who has the most vote becomes the heir
    Elected { term: Term, inaugurationDate: Date },

    /// all members can participate in a fight-to-the-death tournament every decade; the strongest one become leader
    Strongest { lastTournamentDate: Date },
}

pub struct Leadership {
    designation: Designation,
    leader: Option<character::Id>,
}

pub struct Factions {
    owned_buildings: HashMap<FactionId, HashSet<building::Id>>,
}
