use std::collections::HashSet;

use crate::galaxy::living::character;

/// Each "platform" represents a defense structure that serves a certain strategic purposes
pub enum Kind {
    /// a very basic manned tower that boosts reconnaissance; is much less demanding than setting up unmanned platform or satellites
    Watchtower,

    /// an unmanned platform that boosts reconnaissances
    Radar,
}

pub struct Watchtowers {
    level: u8,
    amount: u8,

    /// It should be staffed by a low-rank soldier. Whether this platform needs to be manned depends on the level.
    staff: HashSet<character::Id>,
    manpower: u32,
}

pub struct Platforms {
    watchtower: Watchtowers,
}
