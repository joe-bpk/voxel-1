use raylib::prelude::*;

pub const CHUNKSIZE: usize = 32;
pub const WORLDHEIGHT: usize = 64;
pub const WORLDHEIGHTF32: f32 = WORLDHEIGHT as f32;
pub const WORLDSIZE_CHUNK_REL: usize = 8;
pub const WORLDSIZE_CHUNK: usize = WORLDSIZE_CHUNK_REL * 2;
pub const WORLDSIZE_BLOCKS: usize = WORLDSIZE_CHUNK * CHUNKSIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntVec3
{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IntVec3
{
    pub fn to_rl_vec3(self) -> Vector3
    {
        return Vector3 {
            x: self.x as f32,
            y: self.y as f32,
            z: self.z as f32,
        };
    }

    pub fn zero() -> Self
    {
        Self {
            x: 0, y: 0, z: 0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkLoc
{
    pub loc: IntVec3,
}

impl ChunkLoc
{
    pub fn from_world_loc_rl_vec(vec: Vector3) -> Self
    {
        let chunk_size = CHUNKSIZE as f32;

        Self {
            loc: IntVec3 {
                x: (vec.x / chunk_size).floor() as i32,
                y: (vec.y / chunk_size).floor() as i32,
                z: (vec.z / chunk_size).floor() as i32,
            },
        }
    }

    pub fn to_world_loc(self) -> IntVec3
    {
        let chunk_size = CHUNKSIZE as i32;
        return IntVec3 {
            x: self.loc.x * chunk_size,
            y: self.loc.y * chunk_size,
            z: self.loc.z * chunk_size,
        };
    }

    pub fn compare(&self, other: ChunkLoc) -> bool
    {
        if self.loc.x == other.loc.x && self.loc.z == other.loc.z {
            return true;
        } else {
            return false;
        }
    }
}
