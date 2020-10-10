pub mod character;

pub struct Attributes {
    strength: u8,
    constituion: u8,
    dexterity: u8,
    agility: u8,
    intelligence: u8,
    charisma: u8,
    sense: u8,   // use of external energy
    control: u8, // use of internal energy
    luck: u8,
}

pub enum Sex {
    Male,
    Female,
    Futanari, // have both sexes, need 2 to reproduce
    Asexual,  // reproduce by itself
    Machine,  // don't reproduce naturally
}

pub enum Reproduction {
    Heterosexual, // male & female
    Bisexual,     // reproduce with either male or female
    Asexual,      // reproduce by splitting itself
    Homosexual,   // reproduce by same sex
    Assembly,     // reproduce by mechanical assembly
}

pub enum SocialStructure {
    // normal structure (100% flexible)
    Patriarchy, // leaders are usually male
    Matriarchy, // leaders are usually female

    // extreme/challenge structure (can't switch in and out)
    /**
    - ant-like politics
    - one dominant sex (either male or female)
    - imperial government only
    - lesser sex not represented as characters (though exist in the background)
    */
    AntHive,

    /**
    - synchronized mind
    - all non-machine characters start with telepathy
    - no leader
    - communism only
    */
    Hivemind,

    /**
    - Law of the jungle
    - either uncivilized, living in an apocalypse, or special planets
    - dictatorship only
    */
    Jungle,

    /**
    - one sex only due to genetic modification
    - extremely aggressive & powerful, genocidal
    - need to kidnap others to reproduce
    - natural raiders & can only form tributaries
     */
    MaleOnly,
    FemaleOnly,
}

pub enum Maturity {
    Child,
    Adult,
    Elder,
}

pub struct Life {
    life_energy: u32,
    born: u32,
}
