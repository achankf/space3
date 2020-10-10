use crate::galaxy::{self, living::character};

pub struct Game {
    galaxy: galaxy::Galaxy,
    player: Option<character::Id>,
}
