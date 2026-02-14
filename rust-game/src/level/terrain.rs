use crate::level::utils::*;
use noiselib::*;
use raylib::prelude::*;

#[derive(Copy, Clone)]
struct Block
{
    block_id: usize,
    health:   i32,
}

impl Block
{
    pub fn draw(self, d: &mut RaylibMode3D<RaylibDrawHandle>, loc: Vector3)
    {
        if self.block_id == 1 {
            d.draw_cube(loc, 1.0, 1.0, 1.0, Color::BLUE);
            d.draw_cube_wires(loc, 1.0, 1.0, 1.0, Color::BLACK);
        }
    }
}

pub struct Chunk
{
    chunk_loc: ChunkLoc,
    blocks:    Box<[[[Block; CHUNKSIZE]; WORLDHEIGHT]; CHUNKSIZE]>,
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
        let mut seed = 10;

        let mut rng = noiselib::prelude::UniformRandomGen::new(seed);

        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let mut perlin_out = perlin::perlin_noise_2d(
                        &mut rng,
                        (x as f32 / WORLDSIZE_BLOCKS as f32),
                        (z as f32 / WORLDSIZE_BLOCKS as f32),
                        seed,
                    );

                    let mut block_id = 1;

                    if (perlin_out + 0.5) * WORLDHEIGHT as f32 > y as f32 {
                        block_id = 1;
                    } else {
                        block_id = 0;
                    }
                    self.blocks[x][y][z].block_id = block_id;
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>)
    {
        let mut locRlVec3 = self.chunk_loc.toWorldLoc().toRLVec3();
        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let mut relLoc = Vector3::new(x as f32, y as f32, z as f32);

                    let mut actualLoc = locRlVec3 + relLoc;

                    self.blocks[x][y][z].draw(d, actualLoc);
                }
            }
        }
        d.draw_grid(50, 1.0);
    }
}

pub struct Terrain
{
    chunks: Box<[[Chunk; WORLDSIZE_CHUNK]; WORLDSIZE_CHUNK]>,
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
                    x: x as i32,
                    y: 0,
                    z: z as i32,
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

    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>)
    {
        for x in 0..WORLDSIZE_CHUNK {
            for z in 0..WORLDSIZE_CHUNK {
                self.chunks[x][z].draw(d);
            }
        }
    }
}
