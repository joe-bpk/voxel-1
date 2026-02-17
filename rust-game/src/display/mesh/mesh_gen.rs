use crate::level::terrain::Chunk;
use crate::level::utils::*;
use raylib::prelude::*;

pub fn generate_chunk_mesh(chunk: &Chunk, thread: &RaylibThread) -> Mesh
{
    let mut vertices: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut texcoords: Vec<f32> = Vec::new();

    for x in 0..CHUNKSIZE {
        for y in 0..WORLDHEIGHT {
            for z in 0..CHUNKSIZE {
                let block = &chunk.blocks[x][y][z];

                // Skip air blocks
                if block.block_id == 0 {
                    continue;
                }

                let world_x = x as f32;
                let world_y = y as f32;
                let world_z = z as f32;

                // Check each face and add if exposed
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
    let triangle_count = vertex_count / 3;

    // FIX: Use Mesh::default() or a zero-initialized Mesh instead of gen_mesh_poly
    let mut mesh: Mesh = unsafe { std::mem::zeroed() };

    unsafe {
        // No need to free(mesh.vertices) here because Mesh::default()
        // initializes pointers to null.

        // Allocate new data
        mesh.vertices =
            libc::malloc(vertices.len() * std::mem::size_of::<f32>())
                as *mut f32;
        std::ptr::copy_nonoverlapping(
            vertices.as_ptr(),
            mesh.vertices,
            vertices.len(),
        );

        mesh.normals = libc::malloc(normals.len() * std::mem::size_of::<f32>())
            as *mut f32;
        std::ptr::copy_nonoverlapping(
            normals.as_ptr(),
            mesh.normals,
            normals.len(),
        );

        mesh.texcoords =
            libc::malloc(texcoords.len() * std::mem::size_of::<f32>())
                as *mut f32;
        std::ptr::copy_nonoverlapping(
            texcoords.as_ptr(),
            mesh.texcoords,
            texcoords.len(),
        );

        mesh.vertexCount = vertex_count;
        mesh.triangleCount = triangle_count;

        mesh.upload(false); // This sends the CPU data to the GPU
    }

    mesh
}

/// Check if a face should be rendered (is it exposed to air?)
fn should_render_face(
    chunk: &Chunk,
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

    // Check bounds
    if nx < 0
        || nx >= CHUNKSIZE as i32
        || ny < 0
        || ny >= WORLDHEIGHT as i32
        || nz < 0
        || nz >= CHUNKSIZE as i32
    {
        return true; // Render faces at chunk boundaries
    }

    // Check if neighbor is air
    chunk.blocks[nx as usize][ny as usize][nz as usize].block_id == 0
}

fn add_front_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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

fn add_back_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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

fn add_top_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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

fn add_bottom_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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

fn add_right_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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

fn add_left_face(
    vertices: &mut Vec<f32>,
    normals: &mut Vec<f32>,
    texcoords: &mut Vec<f32>,
    x: f32,
    y: f32,
    z: f32,
)
{
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
