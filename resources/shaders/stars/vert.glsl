#version 330 core

layout (location = 0) in vec3 position;


uniform mat4 view_projection;


void main()
{
    mat3 rot_matrix  = mat3(view_projection);
    vec3 orientation = vec3(view_projection[0][3], view_projection[1][3], view_projection[2][3]);
    float val = dot(orientation, position);
    gl_Position = vec4(rot_matrix * position, val);
    gl_Position.z = val - 0.00001;
}
