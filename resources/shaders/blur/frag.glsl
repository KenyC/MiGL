#version 330 core

in vec2 f_position;

out vec4 color;


uniform sampler2D scene_texture;



void main()
{
	float offset = 1.0 / 150.0;  
	// float kernel[9] = float[](
	// 	-1, -1, -1,
	// 	-1,  9, -1,
	// 	-1, -1, -1
	// );
	float kernel[9] = float[](
		0.0625, 0.125, 0.0625,
		0.125, 0.25, 0.125,
		0.0625, 0.125, 0.0625
	);
	vec2 offsets[9] = vec2[](
		vec2(-offset,  offset), // top-left
		vec2( 0.0f,    offset), // top-center
		vec2( offset,  offset), // top-right
		vec2(-offset,  0.0f),   // center-left
		vec2( 0.0f,    0.0f),   // center-center
		vec2( offset,  0.0f),   // center-right
		vec2(-offset, -offset), // bottom-left
		vec2( 0.0f,   -offset), // bottom-center
		vec2( offset, -offset)  // bottom-right    
	);
	vec3 sampleTex[9];
	for(int i = 0; i < 9; i++)
	{
	    sampleTex[i] = vec3(texture(scene_texture, 0.5 * ((f_position + offsets[i]) + 1.0)));
	}
	vec3 col = vec3(0.0);
	for(int i = 0; i < 9; i++)
	    col += sampleTex[i] * kernel[i];
	
	color = vec4(col, 1.0);
}