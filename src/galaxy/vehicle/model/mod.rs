use crate::entity::Entities;

pub mod air;
pub mod land;
pub mod naval;
pub mod space;

pub type Id = u32;

pub enum Role {
    Land,
    Naval,
    Air,
    Space,
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
    size: Size,
    offence: Offence,
    defense: Defense,
}

pub struct Models {
    entities: Entities<Role, Id>,
    land: land::Models,
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
