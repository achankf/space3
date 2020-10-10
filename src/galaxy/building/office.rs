use std::collections::HashMap;

use crate::{
    entity::Entities,
    galaxy::{corporation, planet::company},
};

use super::Id;

pub enum Purpose {
    Lease {
        // company -> how many floors rented out
        leases: HashMap<company::Id, u8>,
    },
    HQ(corporation::Id),
}

pub struct Office {
    num_floors: u8,
    purpose: Purpose,
}

pub struct Offices {
    offices: Entities<Office, Id>,
}
