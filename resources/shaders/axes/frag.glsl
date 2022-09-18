#version 330 core

in vec4 color_from_vert;

out vec4 color;




void main()
{

    color = color_from_vert;
}