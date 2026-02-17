#version 330

// Input attributes from Raylib Mesh
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;

// Output to Fragment Shader
out vec3 fragPosition;
out vec2 fragTexCoord;
out vec3 fragNormal;
out vec4 fragColor;

// Uniforms provided by Raylib's draw_mesh call
uniform mat4 mvp;      // Model-View-Projection Matrix
uniform mat4 matModel; // Model Matrix (for world-space calculations)

void main()
{
    // Calculate world position for lighting/fog in fragment shader
    fragPosition = vec3(matModel * vec4(vertexPosition, 1.0));

    // Pass along texture coordinates and normals
    fragTexCoord = vertexTexCoord;

    // Transform normals to world space (handles rotation if needed)
    fragNormal = normalize(vec3(matModel * vec4(vertexNormal, 0.0)));

    // Pass the vertex color (this is where AO data usually lives)
    fragColor = vertexColor;

    // Calculate final vertex position on screen
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
