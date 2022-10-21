#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 tex_coords;

out vec3 f_normal;
out vec2 f_tex_coords;
out vec4 world_position;

uniform mat4 model_view;
uniform mat4 projection;


void main()
{
    f_normal = normal;
    world_position = model_view * vec4(position, 1.0);
    f_tex_coords = tex_coords;

    gl_Position = projection * world_position;
}
