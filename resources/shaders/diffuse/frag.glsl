#version 330 core

in float intensity;

out vec4 color;




void main()
{
    color = vec4(intensity * vec3(1.0f, 1.0f, 1.0f), 1.0f);
}