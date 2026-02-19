use crate::level::utils::*;
use noiselib::*;
use terrain_gen::{Block, WorldCfg};

const WORLDCFG: WorldCfg = WorldCfg {
    world_size_b: WORLDSIZE_BLOCKS,
    world_height: WORLDHEIGHT,
    seed:         10,
};

/// # category
/// **client side processing**
///
/// a 3d container for blocks.
#[derive(Clone)]
pub struct Chunk
{
    pub chunk_loc: ChunkLoc,
    pub blocks:    Box<[[[Block; CHUNKSIZE]; WORLDHEIGHT]; CHUNKSIZE]>,
}

impl Chunk
{
    /// creates a new chunk filled with default blocks.
    pub fn new() -> Self
    {
        Self {
            chunk_loc: ChunkLoc {
                loc: IntVec3::zero()
            },
            blocks:    Box::new(
                [[[Block {
                    block_id: 1
                }; CHUNKSIZE]; WORLDHEIGHT]; CHUNKSIZE],
            ),
        }
    }

    /// generates terrain using terrain-gen
    pub fn gen_terr(&mut self)
    {
        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let offset_int = self.chunk_loc.to_world_loc();
                    let b_x = offset_int.x + (x as i32);
                    let b_y = offset_int.y + (y as i32);
                    let b_z = offset_int.z + (z as i32);

                    self.blocks[x][y][z] =
                        terrain_gen::block_gen(b_x, b_y, b_z, WORLDCFG);
                }
            }
        }
    }
}

/// # category
/// **client side processing**
///
/// manager for dynamic world loading and unloading.
pub struct DynTerr
{
    pub chunks: Vec<Chunk>,
}

impl DynTerr
{
    /// initializes an empty terrain manager.
    pub fn new() -> Self
    {
        Self {
            chunks: Vec::new()
        }
    }

    /// retrieves a chunk or generates it if missing.
    pub fn get_chunk(
        &mut self,
        c_loc: ChunkLoc,
    ) -> Result<Chunk, std::io::Error>
    {
        if self.should_gen_chunk(c_loc) {
            let chunk = Self::gen_chunk(c_loc);
            self.chunks.push(chunk.clone());
            return Ok(chunk);
        } else {
            for chunk in &self.chunks {
                if chunk.chunk_loc.compare(c_loc) {
                    return Ok(chunk.clone());
                }
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "chunk not found",
        ))
    }

    /// removes a chunk from memory.
    pub fn deload_chunk(&mut self, c_loc: ChunkLoc) -> bool
    {
        if self.is_chunk_loaded(c_loc) {
            for (idx, chunk) in self.chunks.iter().enumerate() {
                if chunk.chunk_loc.compare(c_loc) {
                    self.chunks.remove(idx);
                    return true;
                }
            }
        }
        false
    }

    /// creates and proceduralizes a new chunk.
    fn gen_chunk(c_loc: ChunkLoc) -> Chunk
    {
        let mut chunk = Chunk::new();
        chunk.chunk_loc = c_loc;
        chunk.gen_terr();
        chunk
    }

    /// checks if chunk exists in persistent storage.
    fn does_chunk_exist(&self, _c_loc: ChunkLoc) -> bool
    {
        false
    }

    /// checks if chunk is currently in ram.
    pub fn is_chunk_loaded(&self, c_loc: ChunkLoc) -> bool
    {
        self.chunks.iter().any(|c| c.chunk_loc.compare(c_loc))
    }

    /// determines if a chunk needs to be generated.
    fn should_gen_chunk(&self, c_loc: ChunkLoc) -> bool
    {
        !self.is_chunk_loaded(c_loc) && !self.does_chunk_exist(c_loc)
    }
}
