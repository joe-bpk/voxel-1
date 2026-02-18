use crate::level::terrain::Chunk;
use crate::level::utils::*;
use raylib::prelude::*; // mesh comes from here now

unsafe extern "C" {
    /// # category
    /// **server side**
    ///
    /// ffi call to the c-based voxel generator.
    ///
    /// this function takes raw vertex data and returns a raylib ffi mesh.
    fn GenerateVoxelMesh(
        vertices: *mut f32,
        normals: *mut f32,
        texcoords: *mut f32,
        vertexCount: i32,
    ) -> raylib::ffi::Mesh; // fully qualified, no import needed
}

/// # category
/// **client side processing**
///
/// generates a raylib-compatible [`Mesh`] from a [`Chunk`].
///
/// this function iterates through every block in a chunk, performs hidden-surface
/// removal (culling faces that touch other blocks), and uploads the resulting
/// geometry to the gpu.
///
/// # safety
///
/// this function calls `GenerateVoxelMesh` via ffi. it assumes the c-side
/// implementation correctly handles the provided pointers before they are dropped
/// by rust at the end of this scope.
pub fn generate_chunk_mesh(chunk: &Chunk, _thread: &RaylibThread) -> Mesh {
    let mut vertices: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut texcoords: Vec<f32> = Vec::new();

    for x in 0..CHUNKSIZE {
        for y in 0..WORLDHEIGHT {
            for z in 0..CHUNKSIZE {
                let block = &chunk.blocks[x][y][z];

                if block.block_id == 0 {
                    continue;
                }

                let world_x = x as f32;
                let world_y = y as f32;
                let world_z = z as f32;

                if should_render_face(chunk, x, y, z, 0, 0, 1) {
                    add_front_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
                if should_render_face(chunk, x, y, z, 0, 0, -1) {
                    add_back_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
                if should_render_face(chunk, x, y, z, 0, 1, 0) {
                    add_top_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
                if should_render_face(chunk, x, y, z, 0, -1, 0) {
                    add_bottom_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
                if should_render_face(chunk, x, y, z, 1, 0, 0) {
                    add_right_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
                if should_render_face(chunk, x, y, z, -1, 0, 0) {
                    add_left_face(
                        &mut vertices,
                        &mut normals,
                        &mut texcoords,
                        world_x,
                        world_y,
                        world_z,
                    );
                }
            }
        }
    }

    let vertex_count = (vertices.len() / 3) as i32;

    unsafe {
        let ffi_mesh = GenerateVoxelMesh(
            vertices.as_mut_ptr(),
            normals.as_mut_ptr(),
            texcoords.as_mut_ptr(),
            vertex_count,
        );

        // the vecs are dropped here — safe because c already memcpy'd them
        std::mem::transmute(ffi_mesh)
    }
}

/// check if a face should be rendered (is it exposed to air?)
fn should_render_face(
    chunk: &Chunk,
    x: usize,
    y: usize,
    z: usize,
    dx: i32,
    dy: i32,
    dz: i32,
) -> bool {
    let nx = x as i32 + dx;
    let ny = y as i32 + dy;
    let nz = z as i32 + dz;

    // check bounds
    if nx < 0
        || nx >= CHUNKSIZE as i32
        || ny < 0
        || ny >= WORLDHEIGHT as i32
        || nz < 0
        || nz >= CHUNKSIZE as i32
    {
        return true; // render faces at chunk boundaries
    }

    // check if neighbor is air
    chunk.blocks[nx as usize][ny as usize][nz as usize].block_id == 0
}

/// adds vertices, normals, and uvs for a front-facing quad (+z).
fn add_front_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x,
        y,
        z + 1.0,
        x + 1.0,
        y,
        z + 1.0,
        x + 1.0,
        y + 1.0,
        z + 1.0,
        x,
        y,
        z + 1.0,
        x + 1.0,
        y + 1.0,
        z + 1.0,
        x,
        y + 1.0,
        z + 1.0,
    ]);

    normals.extend_from_slice(&[
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 1.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}

/// adds vertices, normals, and uvs for a back-facing quad (-z).
fn add_back_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x + 1.0,
        y,
        z,
        x,
        y,
        z,
        x,
        y + 1.0,
        z,
        x + 1.0,
        y,
        z,
        x,
        y + 1.0,
        z,
        x + 1.0,
        y + 1.0,
        z,
    ]);

    normals.extend_from_slice(&[
        0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0,
        0.0, -1.0, 0.0, 0.0, -1.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}

/// adds vertices, normals, and uvs for a top-facing quad (+y).
fn add_top_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x,
        y + 1.0,
        z,
        x,
        y + 1.0,
        z + 1.0,
        x + 1.0,
        y + 1.0,
        z + 1.0,
        x,
        y + 1.0,
        z,
        x + 1.0,
        y + 1.0,
        z + 1.0,
        x + 1.0,
        y + 1.0,
        z,
    ]);

    normals.extend_from_slice(&[
        0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}

/// adds vertices, normals, and uvs for a bottom-facing quad (-y).
fn add_bottom_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x,
        y,
        z + 1.0,
        x,
        y,
        z,
        x + 1.0,
        y,
        z,
        x,
        y,
        z + 1.0,
        x + 1.0,
        y,
        z,
        x + 1.0,
        y,
        z + 1.0,
    ]);

    normals.extend_from_slice(&[
        0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0, -1.0, 0.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}

/// adds vertices, normals, and uvs for a right-facing quad (+x).
fn add_right_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x + 1.0,
        y,
        z + 1.0,
        x + 1.0,
        y,
        z,
        x + 1.0,
        y + 1.0,
        z,
        x + 1.0,
        y,
        z + 1.0,
        x + 1.0,
        y + 1.0,
        z,
        x + 1.0,
        y + 1.0,
        z + 1.0,
    ]);

    normals.extend_from_slice(&[
        1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}

/// adds vertices, normals, and uvs for a left-facing quad (-x).
fn add_left_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
) {
    vertices.extend_from_slice(&[
        x,
        y,
        z,
        x,
        y,
        z + 1.0,
        x,
        y + 1.0,
        z + 1.0,
        x,
        y,
        z,
        x,
        y + 1.0,
        z + 1.0,
        x,
        y + 1.0,
        z,
    ]);

    normals.extend_from_slice(&[
        -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
        0.0, 0.0, -1.0, 0.0, 0.0,
    ]);

    texcoords.extend_from_slice(&[
        0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
    ]);
}
