pub mod region;

use std::collections::HashSet;

use self::region::Region;

use super::Point;

pub type Id = u8;

pub enum Modifier {}

pub enum Kind {
    /// 50% mountain, 20% plain, 30% ocean
    Alpine,
    /// i.e. a planet that resembles Earth; 70% ocean + 20% plain + 10% mountain
    Earth,
    /// 100% Gas
    GasGaint,
    /// 100% ocean
    Ocean,
    /// 100% land
    Barren,
}

pub enum Speciality {
    /** Nothing special */
    None,

    /** Single dominant race on one planet */
    Earth,

    /** Bonus to mineral productions */
    RichMinerals,

    /** Can grow crops all years + bonus yields */
    Evergreen,

    PrecursorMine,

    /**
    - just like how Asia comics work
    - people somehow get a "system" that gives mysterious power
    - portals (represented as realms) spawn as an apocalypse in a developed world and heros fight to close them
    - start with low magic density; increasing as the conflict goes on
    */
    System,

    /**
    - player-only, unique planet in the galaxy
    - one-super-region country with a player-controlled ruler & loyal sidekicks & some super villians
    - lose the game when tha player gets forced out of office (very demanding in people satisfaction)
    - player can change the law at no cost
    - republic only
    - early access to Swiss bank (a mysterious space banking institution)
    */
    ElPrez,

    /**
    - more dominant races spawn in the planet than usual
    - one powerful "evil" artifact
    - 1 powerful immortal dark lord; can only be killed if the evil artifact is destroyed
    - events to build up to gang on the dark lord
    - always mid-fantasy worlds (because Gandalf didn't use magic combat in the movies)
    */
    Tolkienesque,

    /**
    - a surviving, leftover precursor world
    - spawn many explorable realms
    - people resurrect in friendly regions if killed in combat
    - dead "things" drop money
    - mid-high fantasy worlds
    */
    JRPG,

    /**
    - a surviving, leftover precursor world
    - all diseases are cured by the world's bots, perfect contraception (can choose when to get pregnant)
    - very liberal society
    - high tourism from themed parks
    */
    HentaiLogic,

    /**
    - a unique post-apocalyptic world
    - everyone in this planet is as good as Michael Jadeson (sic?)
    - bonus to cultural goods
    - special domesticated zombie dancing "soldiers"
    - all conflicts are resolved by singing & dancing, no violence
    */
    KingOfPop,
}

pub struct Planet {
    kind: Kind,
    regions: Vec<Region>,
    modifiers: HashSet<Modifier>,
    coor: Point,
    mana_density: u8,
}
