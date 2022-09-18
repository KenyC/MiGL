#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;

out float intensity;

uniform mat4   model_view_projection;
uniform vec3   light_direction;
uniform float  min_illumination;
uniform float  max_illumination;

void main()
{
    intensity = - dot(normal, light_direction) * max_illumination;
    if(intensity <= min_illumination) {
        intensity = min_illumination;
    }
    gl_Position = model_view_projection * vec4(position, 1.0);
}
