pub mod terrain;
pub mod utils;

use terrain::Terrain;

use raylib::prelude::*;

pub struct Level
{
    pub terrain: Terrain,
}

impl Level
{
    pub fn new() -> Self
    {
        return Self {
            terrain: Terrain::new(),
        };
    }
    pub fn update(&mut self)
    {
    }
}
