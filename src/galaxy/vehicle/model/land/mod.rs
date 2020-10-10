use std::collections::HashMap;

use enum_map::Enum;

#[derive(Enum)]
pub enum Role {
    Beast,

    // transporation
    Car,   // represent personal transportation for at most 4 named characters
    Truck, // cargo transport
    Train, // efficent cargo transport, but very demanding in infrastructure

    // miliary
    Tank,       // armored
    Missile,    // even get intercepted or explode
    Helicopter, // kill armored units

    // fantasy
    Fortress, // movable fortress + self sustainable settlement
}

#[derive(Clone, Copy)]
pub struct Stats {}

pub struct Model {}

pub struct Models {
    base_models: HashMap<super::Id, Model>,
}
