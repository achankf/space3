use std::collections::HashMap;

pub type Id = u32;

pub struct Role {}

#[derive(Clone, Copy)]
pub struct Stats {}

pub struct Model {}

pub struct Models {
    base_models: HashMap<super::Id, Model>,
}
