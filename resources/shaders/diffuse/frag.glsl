#version 330 core

in vec3 f_normal;
in vec4 world_position;

out vec4 color;

uniform vec3   light_direction;
uniform float  ambient_strength;  // good value : 0.1
uniform float  specular_strength; // good value : 0.1
uniform float  light_strength;
uniform vec3   camera_pos;


void main()
{
    float intensity = 0.0;
    intensity = ambient_strength * light_strength;
    // intensity += max(dot(f_normal, -light_direction), 0.0) * light_strength;
    intensity += max(dot(f_normal, light_direction), 0.0) * light_strength * 0.001;

    vec3 reflected_light_dir = reflect(light_direction, f_normal);
    vec3 f_pt_to_camera = camera_pos - vec3(world_position);
    vec3 dir_pt_to_camera = normalize(f_pt_to_camera);

    intensity += pow(max(dot(reflected_light_dir, dir_pt_to_camera), 0.0), 16) * specular_strength * light_strength;
    // intensity += pow(max(dot(reflected_light_dir, dir_pt_to_camera), 0.0), 32) * specular_strength * light_strength * 0.001;

    color = vec4(intensity * vec3(1.0f, 1.0f, 1.0f), 1.0f);
}