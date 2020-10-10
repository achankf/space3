use crate::galaxy::building::Buildings;

use super::company::Companies;

pub struct Habitat {
    // mutable: max number of buildings in this habitat
    size: u16,
    population_point: u16,
    buildings: Buildings,
    companies: Companies,
}
