use raylib::{ffi::DrawCube, prelude::*};
use crate::level::Level;
use crate::level::terrain::{Chunk, Block}; // Import structs to read data
use crate::level::utils::{CHUNKSIZE, WORLDHEIGHT};

pub struct Display
{
    pub rl: RaylibHandle,
    thread: RaylibThread,
    cam:    Camera3D,
}

impl Display
{
    pub fn new() -> Self
    {
        let (mut rl, thread) = raylib::init().build();
        let cam = Camera3D::perspective(
            Vector3 {
                x: -50.0,
                y: 100.0,
                z: -50.0,
            },
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            45.0,
        );
        return Self {
            rl,
            thread,
            cam,
        };
    }

    // --- RENDER LOGIC START ---

    fn render_block(d: &mut RaylibMode3D<RaylibDrawHandle>, block: &Block, loc: Vector3) {
        if block.block_id == 1 {
            d.draw_cube(loc, 1.0, 1.0, 1.0, Color::BLUE);
            d.draw_cube_wires(loc, 1.0, 1.0, 1.0, Color::BLACK);
        }
    }

    fn render_chunk(d: &mut RaylibMode3D<RaylibDrawHandle>, chunk: &Chunk) {
        let loc_rl_vec3 = chunk.chunk_loc.toWorldLoc().toRLVec3();

        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let block = &chunk.blocks[x][y][z];

                    // Optimization: Don't calculate vector or call draw if air
                    if block.block_id != 0 {
                        let rel_loc = Vector3::new(x as f32, y as f32, z as f32);
                        let actual_loc = loc_rl_vec3 + rel_loc;
                        Self::render_block(d, block, actual_loc);
                    }
                }
            }
        }
    }

    fn draw_3d(d: &mut RaylibMode3D<RaylibDrawHandle>, lvl: &Level)
    {
        // Iterate over terrain chunks and draw them
        // In the future, use lvl.terrain.getChunksToDraw() here
        for row in lvl.terrain.chunks.iter() {
            for chunk in row.iter() {
                Self::render_chunk(d, chunk);
            }
        }

        d.draw_grid(50, 1.0);
    }

    // --- RENDER LOGIC END ---

    pub fn draw_loop(&mut self, lvl: &Level)
    {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        {
            let mut d3d = d.begin_mode3D(self.cam);
            Self::draw_3d(&mut d3d, lvl);
        }

        // 2d drawing
        d.draw_text("Hello from Joe", 20, 20, 20, Color::BLUE);
        d.draw_text(&d.get_fps().to_string(), 20, 40, 20, Color::BLACK );
    }
}
