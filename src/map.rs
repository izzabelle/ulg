use ggez::graphics;
use ggez::Context;
use ggez::GameResult as Result;
use std::collections::HashMap;

pub struct Map {
    data: HashMap<CubeIndex, Tile>,
}

impl Map {
    pub fn new(radius: isize) -> Self {
        let mut data: HashMap<CubeIndex, Tile> = HashMap::new();
        for x in -radius..radius {
            for y in -radius..radius {
                for z in -radius..radius {
                    data.insert((x, y, z).into(), Tile::new(0));
                }
            }
        }
        Self { data }
    }
}

pub struct Tile {
    terrain_texture_idx: usize,
}

impl Tile {
    pub fn new(terrain_texture_idx: usize) -> Self {
        Self { terrain_texture_idx }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CubeIndex {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AxialIndex {
    pub q: isize,
    pub r: isize,
}

impl std::convert::From<(isize, isize, isize)> for CubeIndex {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl std::convert::From<(isize, isize)> for AxialIndex {
    fn from((q, r): (isize, isize)) -> Self {
        Self { q, r }
    }
}

impl std::convert::From<AxialIndex> for CubeIndex {
    fn from(axial_index: AxialIndex) -> Self {
        CubeIndex { x: axial_index.q, y: -axial_index.q - axial_index.r, z: axial_index.r }
    }
}

impl std::convert::From<CubeIndex> for AxialIndex {
    fn from(cube_index: CubeIndex) -> Self {
        AxialIndex { q: cube_index.x, r: cube_index.z }
    }
}
