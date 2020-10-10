pub mod adventurer;
pub mod organized;

use crate::entity::Entities;

use self::organized::Organized;

pub type Id = u32;

pub enum Kind {
    Organized,
}

pub struct Force {}

pub struct Forces {
    organized: Entities<Organized, Id>,
}
