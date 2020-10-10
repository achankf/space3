use crate::entity::Entities;

pub mod naval;
pub mod space;

pub type Id = u32;

pub enum Role {
    Beast, // flying on a beast

    // transporation
    Car,   // represent personal transportation for at most 4 named characters
    Truck, // cargo transport
    Train, // efficent cargo transport, but very demanding in infrastructure

    // miliary
    Tank,       // armored
    Missile,    // even get intercepted or explode
    Helicopter, // kill armored units

    // fantasy
    Fortress, // movable fortress + self sustainable settlemen

    Assault, // specialize in close air support
    Fighter, // air superiority & close air support
    Bomber,  // destroy buildings with bombs
}

/**
Size reference:
- I: personal vehicles to vans, gundams
- II: normal ships, require special infrastructure for land units (i.e. railroad)
- III: normal military ships
- IV: capital naval ships, system-bounded spacecrafts that can dock in planets
- V: "small" independent spacecrafts: cannot land on planets, cross-system travel
- VI: "normal" spacecafts: cost-effective
- VII: "large" spacecrafts
- VIII: "capital" spacecrafts
- IX: "region" scale spacecrafts
- X: "planet" scale spacecrafts
*/
pub type Size = u8;
pub const MAX_VECHICLE_SIZE: Size = 10;

pub struct Offence {
    // beam
    beam_technology: u8,
    beam_quantity: u8,

    // gun
    gun_technology: u8,
    gun_quantity: u8,

    // missile
    missile_technology: u8,
    missile_quantity: u8,
}

pub struct Defense {
    // shield - against beam
    shield_technology: u8,
    shield_quantity: u8,

    // armor - against gun
    armor_technology: u8,
    armor_quantity: u8,

    // countermeasure - against missile
    countermeasure_technology: u8,
    countermeasure_quantity: u8,
}

pub struct Model {
    role: Role,
    size: Size,
    offence: Offence,
    defense: Defense,
}

pub struct Models {
    entities: Entities<Model, Id>,
}

impl Models {
    fn cargo_space(&self, _id: Id) -> u32 {
        todo!()
    }

    fn speed(&self, _id: Id) -> f32 {
        todo!()
    }

    fn weapon_damage(&self, _id: Id) -> f32 {
        todo!()
    }
}
