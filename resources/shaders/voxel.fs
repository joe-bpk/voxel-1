#version 330

/**
 * # category
 * **client side rendering**
 *
 * fragment shader for voxel terrain.
 * handles basic diffuse lighting, ambient sky color, and distance fog.
 */

in vec3 fragPosition;
in vec2 fragTexCoord;
in vec3 fragNormal;
in vec4 fragColor;

out vec4 finalColor;

uniform vec3 lightDir = vec3(0.5, 1.0, 0.2); // direction of the sun
uniform vec4 ambient = vec4(0.3, 0.3, 0.4, 1.0); // sky color

void main() {
    float dotProduct = max(dot(normalize(fragNormal), normalize(lightDir)), 0.0);
    vec4 diffuse = fragColor * dotProduct;

    // simple distance fog
    float dist = length(fragPosition);
    float fogFactor = clamp((dist - 50.0) / 100.0, 0.0, 1.0);
    vec4 fogColor = vec4(1.0, 1.0, 1.0, 1.0);

    finalColor = mix(diffuse + (fragColor * ambient), fogColor, fogFactor);
}
