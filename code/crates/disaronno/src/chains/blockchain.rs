/*
    Create a fully-equipped block structure with a number of standard functions outlined below...
 */
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::blocks::Block;

pub trait Chain {
    type Block;
    type Blocks;

    fn setup(&mut self) -> Self;
    fn connect() -> Result<(), crate::types::BoxedError>;
}


#[derive(Clone, Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

