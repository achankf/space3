use enum_map::Enum;

#[derive(Enum)]
pub enum Commodity {
    // food - prepared per 12 units = lcm(1,2,3,4)
    Crop, // from farms and water; cereal, fruit, etc.
    Meat, // from pastures or fisheries, all flesh from animals
    // food flavoring
    Salt,    // mined from salt pan (found in desert)
    Essence, // extracted from highly magical plants or synthesized from Chemical
    Spice,   // mined from only 1 planet that filled with aggressive giant death sand worms
    // drink
    Water,    // from various ways based on Tech and energy
    Beverage, // from crop

    // raw materials
    Chemical, // from Crop or Metal
    Fiber,    // from Crop
    Polymer,  // from Wood, FossilFuel, Crop, recycled waste
    Glass,    // from nothing; assuming sand is free
    Incense,

    // Fuel
    Wood,       // pre-industrial
    Fuel,       // mid-game
    Antimatter, // late-game

    // minerals
    Gold,  // I, has special economic mechanisms
    Gem,   // I, to jewelry or magic storage
    Metal, // I, represents most metal found on Earth
    Alloy, // I, represents alloys from Metals

    // products
    Apparel,   // from fiber
    Paper,     // from wood, obsolete in late-game
    Accessory, // from wood or gems
    Furniture, // from wood or parts
    Appliance, // from Parts, or (Circuit | Bot) & polymers
    Gadget,    // from (Circuit | Bot) & polymers
    Vehicle,   // from parts & polymers
    Medicine,  // from Herbs, Chemicals, or Bots

    // space travel
    Stillsuit, // from polymer and fiber

    // usable products for manufacturing
    Fertilizer, // from Animal manure & chemicals
    Tool,       // from metal or parts
    Machine,    // from parts
    Printer,    // from (Circuit | Bot) & polymers

    // computer
    Calculator, // from parts; representing counting stones, abacus, mechanical caculator, early electronic calculator
    Circuit,    // early to mid game
    Bot,        // mid to late game

    // replaceable parts / advanced materials
    Plate, // from alloy; gameplay-wise to consolidate metal resources
    Parts, // from wood, metal, alloy
    // construction
    Scaffold,  // from wood
    Structure, // from Plate
}
