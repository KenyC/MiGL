#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;

out vec3 f_color;

uniform mat4 mvp;


void main()
{
	f_color = color;
	gl_Position = mvp * vec4(position, 1.0);
}
