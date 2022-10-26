#version 460 core

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D imgInput;
layout(rgba32f, binding = 1) uniform image2D imgOutput;


uniform vec2 draw;
uniform vec4 draw_color;

void main() {
    //uint index = gl_GlobalInvocationID.z;

    vec2 texelCoord = draw;
    ivec2 offset = ivec2(gl_GlobalInvocationID.xy) - ivec2(gl_NumWorkGroups.x/2,gl_NumWorkGroups.y/2);

    //imageStore(imgInput,ivec2(texelCoord)+offset,draw_color);
    imageStore(imgOutput,ivec2(texelCoord)+offset,draw_color);
    
}
