pub enum Feature {
    NaturalFort, // region controller & allies gain bonus defense
    MineralRich, // bonus metal yield

    // plains-based
    FloodPlain,  // bonus food yield; prone to flooding
    BreadBasket, // bonus food yield
    Swamp,       // more expensive construction cost
    Windy,

    // mountain-based
    MysticMountain, // bonus mana density for the region
    Plateau,        // neglect space penalty from mountain
    MedicalValley,  // bonus essence

    // gas-based
    FuelRich, // bonus fuel yield
}
