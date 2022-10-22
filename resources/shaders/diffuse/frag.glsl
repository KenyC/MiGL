#version 330 core

in vec3 f_normal;
in vec4 world_position;

out vec4 color;

uniform vec3   light_direction;
uniform float  ambient_strength;  // good value : 0.1
uniform float  specular_strength; // good value : 0.1
uniform float  diffuse_strength;
uniform float  light_strength;
uniform vec3   camera_pos;


void main()
{   
    // Critical: normal has been interpolated ; its norm is therefore less than 1
    // approaches one near vertices so you get artefacts around the edges
    vec3 normalized_fnormal = normalize(f_normal);

    // -- AMBIENT LIGHTING
    float intensity = 0.0;
    intensity = ambient_strength * light_strength;

    // -- DIFFUSE LIGHTING
    // intensity += max(dot(f_normal, -light_direction), 0.0) * light_strength;
    intensity += max(dot(normalized_fnormal, -light_direction), 0.0) * diffuse_strength * light_strength;


    // -- SPECULAR LIGHTING
    vec3 reflected_light_dir = reflect(light_direction, normalized_fnormal);
    vec3 f_pt_to_camera = camera_pos - vec3(world_position);
    vec3 dir_pt_to_camera = normalize(f_pt_to_camera);

    intensity += pow(max(dot(reflected_light_dir, dir_pt_to_camera), 0.0), 16) * specular_strength * light_strength;
    // intensity += pow(max(dot(reflected_light_dir, dir_pt_to_camera), 0.0), 32) * specular_strength * light_strength * 0.001;

    // -- COLOR
    color = vec4(intensity * vec3(1.0f, 1.0f, 1.0f), 1.0f);
}