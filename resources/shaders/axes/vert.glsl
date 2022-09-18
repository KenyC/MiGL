#version 330 core

layout (location = 0) in vec3 position;

out vec4 color_from_vert;

uniform mat4 model;
uniform mat4 view_projection;


void main()
{
    if(gl_VertexID >= 4) {
        color_from_vert = vec4(1.0, 0.0, 0.0, 1.0f);
    }
    else if(gl_VertexID >= 2) {
        color_from_vert = vec4(0.0, 1.0, 0.0, 1.0f);
    }
    else {
        color_from_vert = vec4(0.0, 0.0, 1.0, 1.0f);
    }
    gl_Position = view_projection * (model * vec4(position, 1.0));
}
