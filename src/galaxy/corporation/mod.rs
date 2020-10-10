use std::collections::HashMap;

use crate::{
    entity::Entities,
    galaxy::{country, living::character},
};

pub type Id = u32;

pub enum ShareHolder {
    Country(country::Id),
    Coporation(Id),
    Personal(character::Id),
}

pub struct Shares {
    shares_issued: u32,
    share_holders: HashMap<ShareHolder, u32>,
}

pub enum Focus {
    Investment,
    Franchise,
}

pub struct Corporation {
    focus: Focus,
    shares: Shares,
}

pub struct Corporations {
    corporations: Entities<Corporation, Id>,
}
