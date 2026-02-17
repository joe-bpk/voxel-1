#version 330

// Input from vertex shader
in vec3 fragPosition;
in vec2 fragTexCoord;
in vec3 fragNormal;
in vec4 fragColor;

// Output color
out vec4 finalColor;

uniform vec3 lightDir = vec3(0.5, 1.0, 0.2); // Direction of the sun
uniform vec4 ambient = vec4(0.3, 0.3, 0.4, 1.0); // Sky color

void main() {
    float dotProduct = max(dot(normalize(fragNormal), normalize(lightDir)), 0.0);
    vec4 diffuse = fragColor * dotProduct;

    // Simple Distance Fog
    float dist = length(fragPosition);
    float fogFactor = clamp((dist - 50.0) / 100.0, 0.0, 1.0);
    vec4 fogColor = vec4(1.0, 1.0, 1.0, 1.0);

    finalColor = mix(diffuse + (fragColor * ambient), fogColor, fogFactor);
}
