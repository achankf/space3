use std::collections::HashMap;

use crate::galaxy::{
    living::character,
    planet::{self, region},
    Point,
};

use super::vehicle;

pub struct PlanetFoot(planet::Id, Point);

// PlanetRide(planet id, beast's id)
// where the beast's location is PlanetFoot(planet id, coordiate)
pub struct PlanetRide(character::Id);

pub struct PlanetVehicle(vehicle::Id);

pub struct PlanetRegion(planet::Id, region::Id);

pub struct Space(vehicle::Id);

pub struct SpacePerson(Point);

pub enum Kind {
    // planet
    PlanetFoot,
    PlanetRegion,
    PlanetRide,
    PlanetVehicle,

    // space
    SpacePerson,  // i.e. travelling in space using magic (?)
    SpaceRide,    // ride/fly (?) on/inside (?) beasts
    SpaceVehicle, // fly in spacecrafts
}

pub struct Locations {
    planet_foot: HashMap<character::Id, PlanetFoot>,
    planet_vehicle: HashMap<character::Id, PlanetVehicle>,
    planet_region: HashMap<character::Id, PlanetRegion>,
    space: HashMap<character::Id, Space>,
    space_person: HashMap<character::Id, SpacePerson>,
}
