mod farm;
mod hotel;
pub mod office;
mod residence;
pub mod warehouse;
mod workshop;

use self::{hotel::Hotels, residence::Residences};

pub type Id = u8;

pub enum Kind {
    // human buildings
    Residence,
    Hotel,

    // raw material
    Farm,
    Orchard,
    Pasture,
    Fishery,
    Mine,

    // manufacturing
    Workshop,
    Factory,
    Fabricator,
}

pub struct Buildings {
    residences: Residences,
    hotels: Hotels,
}
