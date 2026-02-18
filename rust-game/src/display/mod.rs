use crate::level::terrain::{Block, Chunk};
use crate::level::utils::{CHUNKSIZE, ChunkLoc, WORLDHEIGHT, WORLDHEIGHTF32};

use raylib::prelude::*;

pub mod mesh;

use crate::display::mesh::*;

pub const RENDER_DISTANCE: usize = 8;
pub const REND_DIST_BLOCKS: usize = RENDER_DISTANCE * CHUNKSIZE;

/// A Display struct for client side rendering
pub struct Display
{
    chunk_meshes: Vec<ChunkMesh>,
    shader:       Shader,
    pub rl:       RaylibHandle,
    thread:       RaylibThread,
    pub cam:      Camera3D,
}

impl Display
{
    pub fn new() -> Self
    {
        let (mut rl, thread) = raylib::init().build();
        rl.set_window_size(1600, 900);
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
        let chunk_meshes: Vec<ChunkMesh> = Vec::new();
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

    pub fn load_chunk(&mut self, chunk: &Chunk)
    {
        self.chunk_meshes.push(ChunkMesh::gen_from_chunk(
            &mut self.rl,
            &self.thread,
            chunk,
            &self.shader,
            FFI_RED,
        ))
    }

    pub fn render_chunk_meshs(
        cam: &Camera3D,
        d: &mut RaylibMode3D<RaylibDrawHandle>,
        meshes: &[ChunkMesh],
    )
    {
        for chunk in meshes {
            if Self::is_chunk_visible(cam, chunk.position) {
                chunk.draw(d);
            }
        }
    }

    fn is_chunk_visible(cam: &Camera3D, chunk_pos: Vector3) -> bool
    {
        let chunk_size = CHUNKSIZE as f32;
        let chunk_center = Vector3::new(
            chunk_pos.x + chunk_size / 2.0,
            chunk_pos.y + WORLDHEIGHT as f32 / 2.0,
            chunk_pos.z + chunk_size / 2.0,
        );

        // Distance culling
        let distance = (chunk_center - cam.position).length();
        if distance > REND_DIST_BLOCKS as f32 {
            return false;
        }

        // View direction culling
        let to_chunk = (chunk_center - cam.position).normalized();
        let cam_forward = (cam.target - cam.position).normalized();
        to_chunk.dot(cam_forward) > -0.2 // Don't render behind camera
    }

    pub fn is_chunk_loaded(&self, chunk_pos: ChunkLoc) -> bool
    {
        for chunk in &self.chunk_meshes {
            if chunk.chunk_loc.compare(chunk_pos) {
                return true;
            }
        }

        return false;
    }

    // --- RENDER LOGIC END ---

    pub fn draw_loop(&mut self)
    {
        self.rl
            .update_camera(&mut self.cam, CameraMode::CAMERA_FREE);
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::DARKBLUE);

        {
            let mut d3d = d.begin_mode3D(self.cam);

            Self::render_chunk_meshs(&self.cam, &mut d3d, &self.chunk_meshes);
        }

        d.draw_text("Hello from Joe", 20, 20, 20, Color::BLUE);
        d.draw_text(&d.get_fps().to_string(), 20, 40, 20, Color::BLACK);
    }
}
