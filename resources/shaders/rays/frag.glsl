#version 330 core


in vec4 f_color;
out vec4 color;




void main()
{
    // color = vec4(1.0, 0.0, 0.0, 1.0);
    color = f_color;
}