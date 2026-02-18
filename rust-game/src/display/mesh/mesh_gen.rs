use crate::level::terrain::Chunk;
use crate::level::utils::*;
use raylib::prelude::*; // mesh comes from here now

/// holds references to the 4 cardinal neighbor chunks.
/// used to check for solid blocks across chunk boundaries.
pub struct ChunkNeighbors<'a>
{
    pub pos_x: Option<&'a Chunk>,
    pub neg_x: Option<&'a Chunk>,
    pub pos_z: Option<&'a Chunk>,
    pub neg_z: Option<&'a Chunk>,
}

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

/// table definition for face generation.
/// each entry contains:
/// 1. neighbor direction [dx, dy, dz]
/// 2. face normal [nx, ny, nz]
/// 3. vertex offsets [x1, y1, z1, x2, y2, z2, ...] (18 floats for 2 triangles)
#[rustfmt::skip]
const FACE_DATA: [([i32; 3], [f32; 3], [f32; 18]); 6] = [
    // front (+z)
    (
        [0, 0, 1],       // direction to check
        [0.0, 0.0, 1.0], // normal
        [
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, // tri 1
            0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, // tri 2
        ],
    ),
    // back (-z)
    (
        [0, 0, -1],
        [0.0, 0.0, -1.0],
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, // tri 1
            1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, // tri 2
        ],
    ),
    // top (+y)
    (
        [0, 1, 0],
        [0.0, 1.0, 0.0],
        [
            0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, // tri 1
            0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, // tri 2
        ],
    ),
    // bottom (-y)
    (
        [0, -1, 0],
        [0.0, -1.0, 0.0],
        [
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // tri 1
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // tri 2
        ],
    ),
    // right (+x)
    (
        [1, 0, 0],
        [1.0, 0.0, 0.0],
        [
            1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // tri 1
            1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, // tri 2
        ],
    ),
    // left (-x)
    (
        [-1, 0, 0],
        [-1.0, 0.0, 0.0],
        [
            0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, // tri 1
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, // tri 2
        ],
    ),
];

/// standard texture coordinates for a quad (0,0 to 1,1)
#[rustfmt::skip]
const QUAD_TEXCOORDS: [f32; 12] = [
    0.0, 1.0, 1.0, 1.0, 1.0, 0.0, // tri 1
    0.0, 1.0, 1.0, 0.0, 0.0, 0.0, // tri 2
];

/// # category
/// **client side processing**
///
/// generates a raylib-compatible [`Mesh`] from a [`Chunk`].
///
/// this function iterates through every block in a chunk, performs
/// hidden-surface removal (culling faces that touch other blocks), and uploads
/// the resulting geometry to the gpu.
///
/// # safety
///
/// this function calls `GenerateVoxelMesh` via ffi. it assumes the c-side
/// implementation correctly handles the provided pointers before they are
/// dropped by rust at the end of this scope.
pub fn generate_chunk_mesh(
    chunk: &Chunk,
    neighbors: &ChunkNeighbors,
    _thread: &RaylibThread,
) -> Mesh
{
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

                // iterate over all 6 directions defined in the table
                for (dir, normal, v_offsets) in &FACE_DATA {
                    if should_render_face(
                        chunk, neighbors, x, y, z, dir[0], dir[1], dir[2],
                    ) {
                        // push vertices for this face
                        // we iterate 0..6 because each face has 6 vertices (2
                        // triangles)
                        for i in 0..6 {
                            vertices.push(world_x + v_offsets[i * 3]);
                            vertices.push(world_y + v_offsets[i * 3 + 1]);
                            vertices.push(world_z + v_offsets[i * 3 + 2]);
                        }

                        // push normals (same normal for all 6 vertices of the
                        // face)
                        for _ in 0..6 {
                            normals.extend_from_slice(normal);
                        }

                        // push texcoords
                        texcoords.extend_from_slice(&QUAD_TEXCOORDS);
                    }
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
    neighbors: &ChunkNeighbors,
    x: usize,
    y: usize,
    z: usize,
    dx: i32,
    dy: i32,
    dz: i32,
) -> bool
{
    let nx = x as i32 + dx;
    let ny = y as i32 + dy;
    let nz = z as i32 + dz;

    // in-bounds: check this chunk
    if nx >= 0
        && nx < CHUNKSIZE as i32
        && ny >= 0
        && ny < WORLDHEIGHT as i32
        && nz >= 0
        && nz < CHUNKSIZE as i32
    {
        return chunk.blocks[nx as usize][ny as usize][nz as usize].block_id
            == 0;
    }

    // out-of-bounds: check the neighboring chunk if available
    // note: vertical bounds (ny) are world limits, not chunk limits
    if ny < 0 || ny >= WORLDHEIGHT as i32 {
        return true; // always render top/bottom of world
    }

    let (neighbor, local_x, local_z) = if nx < 0 {
        (neighbors.neg_x.as_ref(), CHUNKSIZE as i32 - 1, nz)
    } else if nx >= CHUNKSIZE as i32 {
        (neighbors.pos_x.as_ref(), 0, nz)
    } else if nz < 0 {
        (neighbors.neg_z.as_ref(), nx, CHUNKSIZE as i32 - 1)
    } else if nz >= CHUNKSIZE as i32 {
        (neighbors.pos_z.as_ref(), nx, 0)
    } else {
        return true; // should be unreachable given in-bounds check above
    };

    match neighbor {
        Some(n) => {
            n.blocks[local_x as usize][ny as usize][local_z as usize].block_id
                == 0
        }
        None => true, // neighbor not loaded yet, render the face to be safe
    }
}
