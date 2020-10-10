pub const MAX_REGIONS_PER_PLANET: u8 = 12;
pub const MAX_PLANETS_IN_UNIVERSE: u8 = 100;
pub const MAX_REGIONS_IN_UNIVERSE: u16 =
    (MAX_REGIONS_PER_PLANET as u16) * (MAX_PLANETS_IN_UNIVERSE as u16);