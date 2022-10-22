#version 330 core

layout (location = 0) in vec3 pos;




void main()
{

    gl_Position = vec4(pos, 1.0);
}


/* 

M t 
c 1  (λdir + c; 1)

λMdir + Mc + t
λ<dir, c> + 1





 */