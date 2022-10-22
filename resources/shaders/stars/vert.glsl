#version 330 core

layout (location = 0) in vec3  position;
layout (location = 1) in uint  constellation;
layout (location = 2) in float magnitude;


out vec4 f_color;

uniform mat4 view_projection;
uniform uint current_constellation;
uniform float min_magnitude;
uniform float max_magnitude;


void main()
{
    mat3 rot_matrix  = mat3(view_projection);
    vec3 orientation = vec3(view_projection[0][3], view_projection[1][3], view_projection[2][3]);
    float val = dot(orientation, position);

    if (constellation == current_constellation) {
        gl_PointSize = 2.0;
        f_color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    else {
        gl_PointSize = 1.0;
        f_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    f_color *= max((magnitude - min_magnitude) / (max_magnitude - min_magnitude), 0.25);

    gl_Position = vec4(rot_matrix * position, val);
    gl_Position.z = val - 0.00001;
}
