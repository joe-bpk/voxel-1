mod mesh_gen;

use crate::display::mesh::mesh_gen::*;
use crate::level::terrain::Chunk;
use crate::level::utils::*;
use raylib::prelude::*;

/// # category
/// **client side rendering**
///
/// a representation of a terrain chunk's visual data in the gpu.
///
/// `chunkmesh` acts as the bridge between raw voxel data and raylib's
/// rendering system, storing the compiled mesh and its associated material.
pub struct ChunkMesh
{
    pub mesh:      Mesh,
    pub mat:       WeakMaterial,
    pub is_loaded: bool,
    pub position:  Vector3,
    pub chunk_loc: ChunkLoc,
}

pub const FFI_RED: raylib::ffi::Color = raylib::ffi::Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

impl ChunkMesh
{
    pub fn gen_from_chunk(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        chunk: &Chunk,
        shader: &Shader,
        color: raylib::ffi::Color,
    ) -> Self
    {
        return Self {
            mesh:      generate_chunk_mesh(chunk, thread),
            mat:       Self::color_to_material(rl, thread, shader, color),
            is_loaded: true,
            position:  chunk.chunk_loc.to_world_loc().to_rl_vec3(),
            chunk_loc: chunk.chunk_loc,
        };
    }

    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>)
    {
        let mat = self.mat.clone();
        let mut matrix = Matrix::translate(
            self.position.x,
            self.position.y,
            self.position.z,
        );
        d.draw_mesh(&self.mesh, mat, matrix);
    }

    fn color_to_material(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        shader: &Shader,
        color: raylib::ffi::Color,
    ) -> WeakMaterial
    {
        let mut material = rl.load_material_default(thread);

        // raylib-rs Shaders and Materials can be dereferenced to their FFI structs
        use std::ops::Deref;
        unsafe {
            let mat_ptr: *mut raylib::ffi::Material = material.as_mut();
            (*mat_ptr).shader = *shader.deref();
        }

        material.maps_mut()[0].color = color;
        material
    }
}
