use crate::level::Level;
use crate::level::terrain::{Block, Chunk}; // Import structs to read data
use crate::level::utils::{CHUNKSIZE, WORLDHEIGHT, WORLDHEIGHTF32};


use raylib::{ffi::DrawCube, prelude::*};

pub mod mesh;

use crate::display::mesh::*;

pub const RENDER_DISTANCE: usize = 16;
pub const REND_DIST_BLOCKS: usize = RENDER_DISTANCE * CHUNKSIZE;

pub struct Display
{
    pub rl:       RaylibHandle,
    thread:       RaylibThread,
    cam:          Camera3D,
    chunk_meshes: Vec<ChunkMesh>,
    shader:       Shader,
}

impl Display
{
    pub fn new() -> Self
    {
        let (mut rl, thread) = raylib::init().build();
        let cam = Camera3D::perspective(
            Vector3 {
                x: -10.0,
                y: WORLDHEIGHTF32,
                z: -10.0,
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
        rl.disable_cursor();
        let mut chunk_meshes: Vec<ChunkMesh> = Vec::new();
        let shader = rl.load_shader(
            &thread,
            Some("resources/shaders/voxel.vs"),
            Some("resources/shaders/voxel.fs"),
        );
        return Self {
            rl,
            thread,
            cam,
            chunk_meshes,
            shader,
        };
    }

    // --- RENDER LOGIC START ---

    fn render_block(
        d: &mut RaylibMode3D<RaylibDrawHandle>,
        block: &Block,
        loc: Vector3,
    )
    {
        if block.block_id == 1 {
            d.draw_cube(loc, 1.0, 1.0, 1.0, Color::BLUE);
            d.draw_cube_wires(loc, 1.0, 1.0, 1.0, Color::BLACK);
        }
    }

    pub fn load_chunks(&mut self, chunks: &Vec<Chunk>)
    {
        for chunk in chunks {
            self.chunk_meshes.push(ChunkMesh::genFromChunk(
                &mut self.rl,
                &self.thread,
                &chunk,
                &self.shader,
                FFI_RED,
            ))
        }
    }

    pub fn render_chunk_meshs(
        cam: &Camera3D,
        d: &mut RaylibMode3D<RaylibDrawHandle>,
        meshes: &[ChunkMesh], // Pass the meshes as a slice
    )
    {
        for chunk in meshes {
            if Self::is_chunk_visible(cam, chunk.position) {
                chunk.draw(d);
            }
        }
    }

    fn is_chunk_visible(cam: &Camera3D, chunk_pos: Vector3) -> bool {
        let chunk_size = CHUNKSIZE as f32;
        let chunk_center = Vector3::new(
            chunk_pos.x + chunk_size / 2.0,
            chunk_pos.y + WORLDHEIGHT as f32 / 2.0,
            chunk_pos.z + chunk_size / 2.0,
        );

        // Distance culling
        let distance = (chunk_center - cam.position).length();
        if distance > REND_DIST_BLOCKS as f32 { return false; }

        // View direction culling
        let to_chunk = (chunk_center - cam.position).normalized();
        let cam_forward = (cam.target - cam.position).normalized();
        to_chunk.dot(cam_forward) > -0.2 // Don't render behind camera
    }

    fn render_chunk(d: &mut RaylibMode3D<RaylibDrawHandle>, chunk: &Chunk)
    {
        let loc_rl_vec3 = chunk.chunk_loc.toWorldLoc().toRLVec3();

        for x in 0..CHUNKSIZE {
            for y in 0..WORLDHEIGHT {
                for z in 0..CHUNKSIZE {
                    let block = &chunk.blocks[x][y][z];

                    // Optimization: Don't calculate vector or call draw if air
                    if block.block_id != 0 {
                        let rel_loc =
                            Vector3::new(x as f32, y as f32, z as f32);
                        let actual_loc = loc_rl_vec3 + rel_loc;
                        Self::render_block(d, block, actual_loc);
                    }
                }
            }
        }
    }

    pub fn load_lvl(&mut self, lvl: &Level)
    {
        let mut chunks: Vec<Chunk> = Vec::new();
        for row in lvl.terrain.chunks.iter() {
            for chunk in row.iter() {
                chunks.push(chunk.clone());
            }
        }

        self.load_chunks(&chunks)
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

    pub fn draw_loop(&mut self)
    {
        self.rl
            .update_camera(&mut self.cam, CameraMode::CAMERA_FREE);
        // 1. Start the drawing (borrows self.rl and self.thread)
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::DARKBLUE);

        {
            let mut d3d = d.begin_mode3D(self.cam);

            // 2. Call the updated logic.
            // We pass the meshes directly from self.
            Self::render_chunk_meshs(&self.cam, &mut d3d, &self.chunk_meshes);
        }

        d.draw_text("Hello from Joe", 20, 20, 20, Color::BLUE);
        d.draw_text(&d.get_fps().to_string(), 20, 40, 20, Color::BLACK);
    }
    pub fn draw_loop_old(&mut self, lvl: &Level)
    {
        self.rl
            .update_camera(&mut self.cam, CameraMode::CAMERA_FIRST_PERSON);
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        unsafe {
            raylib::ffi::rlEnableBackfaceCulling();
        }

        {
            let mut d3d = d.begin_mode3D(self.cam);
            Self::draw_3d(&mut d3d, lvl);
        }

        // 2d drawing
        d.draw_text("Hello from Joe", 20, 20, 20, Color::BLUE);
        d.draw_text(&d.get_fps().to_string(), 20, 40, 20, Color::BLACK);
    }
}
