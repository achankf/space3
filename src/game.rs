use cgmath::Point2;
use std::collections::{HashMap, HashSet};

pub type DynastyId = u32;
pub type CharacterId = u64;
pub type PlanetId = u8;
pub type RegionId = u8;
pub type BuildingId = u8;

pub struct CharacterAttributes {
    strength: u8,
    constituion: u8,
    dexterity: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
    luck: u8,
}

pub enum CharacterSkills {
    Labour,
    Hacking,
    Social,
    Researching,
}

pub struct Character {
    dynasty_id: Option<DynastyId>,
    habitability_indicator: u8,
    attributes: CharacterAttributes,
}

pub struct FamilyStandings {}

pub struct Dynasty {
    parent_dynasty: Option<DynastyId>,
    living_members: HashMap<CharacterId, FamilyStandings>,

    prestige: u32,
}

pub enum PlanetModifier {}

pub struct Planet {
    /// Determines native character's habitability indicator at birth.
    /// If a character is born in a dome, then that character follows
    /// the dome's indicator.
    habitability_indicator: u8,
    regions: HashMap<Point2<f32>, Region>,
    modifiers: HashSet<PlanetModifier>,
}

pub enum Product {
    // raw materials
    Crop,  // to food, chemical (seasonal high-yield harvest)
    Metal, // to vehicles, machines, weapons
    Gem,   // to accessory, weapons
    Fuel,  // fuel for spacecraft, power plant

    // intermediate
    Fiber,    // to apparels, from crops
    Chemical, // to medicines & hulls, from any raw materials
    Circuit,  // to gadgets, computers, from metals
    Computer, // from circuits

    // common goods
    Food,     // generic food, from animals or crops
    Drink,    // from crops
    Apparel,  // from fibers
    Medicine, // from chemicals

    // luxuary
    Accessory, // from gems
    Furniture, // from fiber
    Gadget,    // from computers
    Vehicle,   // from metals

    // operational
    // Concrete, // construction, from metal
    Machine, // from metal and computers, used by industries
    Tool,    // from metal, used for raw material production
}

pub enum BuildingKind {
    Residence,
    Hotel,
}

pub enum RegionKind {
    Plain,
    Ocean,
    Mountain,
    Gas,
}

/// characters can live in a habitat when
/// range1 and range2 overlap, where
/// range1 = character.habitability_indicator ± character.habitat_compatibility
/// range2 = habitat.habitability_indicator ± habitat.habitat_compatibility
/// https://stackoverflow.com/a/3269471
pub struct Habitability {
    indicator: u8,
    compatibility: u8,
}

pub struct Habitat {
    habitability: Habitability,
    // mutable: max number of buildings in this habitat
    size: u8,
    population_point: u16,
    buildings: HashMap<BuildingKind, HashSet<BuildingId>>,
}

pub enum RegionModifier {}

pub struct Region {
    kind: RegionKind,
    /// immutable: max number of buildings in the habitat
    landmass: u8,
    habitat: Option<Habitat>,
    base_scenery: u8,
    devastation: u16,
    modifiers: HashSet<RegionModifier>,
}

pub struct Characters {
    entities: Vec<Character>,
    heirs: HashMap<CharacterId, CharacterId>,
    residence: HashMap<CharacterId, (PlanetId, RegionId, BuildingId)>,
}

pub struct Game {
    dynasties: Vec<Dynasty>,
    characters: Characters,
    planets: Vec<Planet>,
}
