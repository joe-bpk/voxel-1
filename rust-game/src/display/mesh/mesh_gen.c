#include "raylib.h"
#include <stdlib.h>
#include <string.h>

/**
 * # category
 * **server side**
 * * c-backend for high-performance mesh generation.
 *
 * this function handles the raw memory allocation and data transfer
 * from rust-managed vectors to raylib-managed gpu buffers.
 */

/**
 * creates a raylib mesh from raw vertex, normal, and texture data.
 *
 * # memory management
 * this function performs a deep copy of the input arrays using `malloc`.
 * the resulting mesh must be freed using raylib's `UnloadMesh` to prevent
 * memory leaks.
 */
Mesh GenerateVoxelMesh(float* vertices, float* normals, float* texcoords, int vertexCount) {
    Mesh mesh = { 0 };
    mesh.vertexCount = vertexCount;
    mesh.triangleCount = vertexCount / 3;

    // allocate memory using standard c malloc
    // raylib's unloadmesh will eventually call free() on these
    int vertSize = vertexCount * 3 * sizeof(float);
    int texSize = vertexCount * 2 * sizeof(float);

    mesh.vertices = (float*)malloc(vertSize);
    memcpy(mesh.vertices, vertices, vertSize);

    mesh.normals = (float*)malloc(vertSize);
    memcpy(mesh.normals, normals, vertSize);

    mesh.texcoords = (float*)malloc(texSize);
    memcpy(mesh.texcoords, texcoords, texSize);

    // upload to gpu memory immediately
    // this finalizes the mesh so it is ready for the client-side rendering call
    UploadMesh(&mesh, false);

    return mesh;
}
