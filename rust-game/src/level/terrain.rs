use crate::level::utils::*;
use noiselib::*;

// Removed Raylib drawing imports, keeping only basic types if needed
// strictly for generation, though usually we'd remove raylib here entirely.

#[derive(Copy, Clone)]
pub struct Block
{
    pub block_id: usize,
    pub health:   i32,
}

// Block logic only (no drawing)
impl Block
{
    // Any logic regarding block health/interaction would go here
}

#[derive(Clone)]
pub struct Chunk
{
    pub chunk_loc: ChunkLoc,
    pub blocks:    Box<[[[Block; CHUNKSIZE]; WORLDHEIGHT]; CHUNKSIZE]>,
}

impl Chunk
{
    pub fn new() -> Self
    {
        Self {
            chunk_loc: ChunkLoc {
                loc: IntVec3::zero(),
            },
            blocks:    Box::new(
                [[[Block {
                    health:   100,
                    block_id: 1,
                }; CHUNKSIZE]; WORLDHEIGHT]; CHUNKSIZE],
            ),
        }
    }

    pub fn perlinify(&mut self)
    {
        let seed = 10;
        let mut rng = noiselib::prelude::UniformRandomGen::new(seed);

        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let mut offset = self.chunk_loc.toWorldLoc().toRLVec3();
                    let perlin_out = musgrave::musgrave_noise_2d(
                        &mut rng,
                        ((x as f32 + offset.x) / WORLDSIZE_BLOCKS as f32),
                        ((z as f32 + offset.z) / WORLDSIZE_BLOCKS as f32),
                        seed,
                    );

                    let block_id;

                    let mut perlin_out_normal = (perlin_out + 1.0)/2.0;

                    if (perlin_out_normal*WORLDHEIGHTF32) > y as f32 {
                        block_id = 1;
                    } else {
                        block_id = 0;
                    }
                    self.blocks[x][y][z].block_id = block_id;
                }
            }
        }
    }
}

pub struct Terrain
{
    pub chunks: Box<[[Chunk; WORLDSIZE_CHUNK]; WORLDSIZE_CHUNK]>,
}

impl Terrain
{
    pub fn new() -> Self
    {
        // Create chunks on the heap
        let mut chunks: Box<[[Chunk; WORLDSIZE_CHUNK]; WORLDSIZE_CHUNK]> =
            Box::new(std::array::from_fn(|_| {
                std::array::from_fn(|_| Chunk::new())
            }));

        for x in 0..WORLDSIZE_CHUNK {
            for z in 0..WORLDSIZE_CHUNK {
                chunks[x][z].chunk_loc.loc = IntVec3 {
                    x: (x as i32 - WORLDSIZE_CHUNK_REL as i32),
                    y: 0,
                    z: (z as i32 - WORLDSIZE_CHUNK_REL as i32),
                };
            }
        }

        return Self {
            chunks: chunks
        };
    }

    pub fn perlinify(&mut self)
    {
        for x in 0..WORLDSIZE_CHUNK {
            for z in 0..WORLDSIZE_CHUNK {
                self.chunks[x][z].perlinify();
            }
        }
    }

    // Logic to get relevant data, but NOT draw it
    pub fn get_loaded_chunks(&self) -> Vec<&Chunk> {
        let mut loaded = Vec::new();
        for x in 0..WORLDSIZE_CHUNK {
            for z in 0..WORLDSIZE_CHUNK {
                loaded.push(&self.chunks[x][z]);
            }
        }
        loaded
    }
}
