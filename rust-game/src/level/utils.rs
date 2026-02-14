use raylib::prelude::*;

pub const CHUNKSIZE: usize = 32;
pub const WORLDHEIGHT: usize = 64;
pub const WORLDSIZE_CHUNK: usize = 4;
pub const WORLDSIZE_BLOCKS: usize = WORLDSIZE_CHUNK * CHUNKSIZE;

pub struct PlayerInfo {
    pub loc: Vector3,
    pub render_distance: usize
}

impl PlayerInfo {
    pub fn get_chunk_loc(&self) -> IntVec3
    {
        let self_loc_x = self.loc.x - (self.loc.x % 32.0);
        let self_loc_y = 0.0;
        let self_loc_z = self.loc.z - (self.loc.z % 32.0);
        return IntVec3 {
            x: self_loc_x as i32 / CHUNKSIZE as i32,
            y: self_loc_y as i32 / CHUNKSIZE as i32,
            z: self_loc_z as i32 / CHUNKSIZE as i32,
        }
    }
}

#[derive(Copy, Clone)]
pub struct IntVec3
{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IntVec3
{
    pub fn toRLVec3(self) -> Vector3
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

#[derive(Copy, Clone)]
pub struct ChunkLoc
{
    pub loc: IntVec3,
}

impl ChunkLoc
{
    pub fn toWorldLoc(self) -> IntVec3
    {
        let chunk_size = CHUNKSIZE as i32;
        // FIX: Used to use self.loc.x for y and z as well
        return IntVec3 {
            x: self.loc.x * chunk_size,
            y: self.loc.y * chunk_size,
            z: self.loc.z * chunk_size,
        };
    }
}
