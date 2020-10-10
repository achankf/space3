use enum_map::Enum;

// note: refuel before flight, but each aircraft has a mission range that decides how
// long the planes stay in the mission, so they won't crash due to running out of fuel

#[derive(Enum)]
pub enum Role {
    Beast,   // flying on a beast
    Assault, // specialize in close air support
    Fighter, // air superiority & close air support
    Bomber,  // destroy buildings with bombs
}

#[derive(Clone, Copy)]
pub struct Stats {}
