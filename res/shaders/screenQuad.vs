#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;


uniform mat4 proj;
uniform mat4 view;
uniform mat4 model;
out vec2 TexCoords;
	
void main()
{
    TexCoords = aTexCoords;
    gl_Position = proj*view*model*vec4(aPos, 1.0);
}
