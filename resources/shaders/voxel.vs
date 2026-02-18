#version 330

/**
 * # category
 * **client side rendering**
 *
 * vertex shader for voxel terrain.
 * transforms vertex data into world space and passes attributes to the fragment shader.
 */

in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;

out vec3 fragPosition;
out vec2 fragTexCoord;
out vec3 fragNormal;
out vec4 fragColor;

uniform mat4 mvp;      // model-view-projection matrix
uniform mat4 matModel; // model matrix for world-space calculations

void main()
{
    // calculate world position for lighting/fog
    fragPosition = vec3(matModel * vec4(vertexPosition, 1.0));
    fragTexCoord = vertexTexCoord;

    // transform normals to world space
    fragNormal = normalize(vec3(matModel * vec4(vertexNormal, 0.0)));
    fragColor = vertexColor;

    // final screen position
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
