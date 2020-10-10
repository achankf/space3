/*!
# References
- expose immutable function that internally mutate cache: https://doc.rust-lang.org/std/cell/

# Limits
- max number of planets: 57
- max number of regions per planet: 10
- max number of dynasties per region per planet: 12
- max number of stations: 115
- max number of dynasties per station: 6
- max member per dynasty: 6
- free characters: 20356
- theoretical max number of characters in the galaxy: ((57 * 10 * 12) + (115 * 6)) * 6 + 20356 = 65536 = 2^16
- ((2^16-num_free_characters)/6-(115*6))/12/10 = max_planets
*/

pub mod building;
pub mod commodity;
pub mod constants;
pub mod corporation;
pub mod country;
pub mod faction;
pub mod living;
pub mod location;
pub mod military;
pub mod planet;
pub mod skills;
pub mod vehicle;

use self::{
    building::Buildings,
    faction::Factions,
    living::{character::Characters, dynasty::Dynasties},
    planet::Planet,
};

pub type Date = u64;

// 2D game
pub type Point = cgmath::Point2<f32>;
pub type Vector = cgmath::Vector2<f32>;

pub struct Galaxy {
    dynasties: Dynasties,
    characters: Characters,
    planets: Vec<Planet>,
    factions: Factions,
    buildings: Buildings,
}
