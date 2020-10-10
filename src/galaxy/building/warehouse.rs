use std::collections::HashSet;

use enum_map::EnumMap;

use crate::galaxy::commodity::Commodity;

pub enum Module {
    Refrigeration,
}

pub struct Storage {
    quantity: u32,
    scale: u32,
    modules: HashSet<Module>,
}

pub struct Warehouse {
    storages: EnumMap<Commodity, Storage>,
}
