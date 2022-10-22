#version 330 core

layout (points) in;
layout (line_strip, max_vertices = 2) out;


uniform mat4 view_projection;
uniform vec3 ray_dir;


out vec4 f_color;

void main()
{
    f_color = vec4(0.1, 0.1, 0.1, 1.0);
    gl_Position = view_projection * (gl_in[0].gl_Position + 10 * vec4(ray_dir, 0.0)); 
    EmitVertex();

    f_color = vec4(1.0, 1.0, 1.0, 1.0);
    gl_Position = view_projection * (gl_in[0].gl_Position - 10 * vec4(ray_dir, 0.0)); 
    EmitVertex();
    
    
    EndPrimitive();

}