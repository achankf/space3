use enum_map::Enum;

#[derive(Enum)]
pub enum Role {
    Beast,

    Boat,   // personal transport, at most 10 people
    Cargo,  // general-purpose cargo transport, loaded in containers
    Tanker, // fuel-only

    // miliary
    Transport,  // transport troops & support amphibious landing
    Corvette,   // basic warship, preferred by pirates
    Destroyer,  // backbone & meat shield for capital ships
    Submarine,  // kill ships & shoot missiles
    Battleship, // capital ship until carrier takes over
    Carrier,    // capital ship
}

#[derive(Clone, Copy)]
pub struct Stats {}
