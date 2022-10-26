#version 460 core

layout (local_size_x = 30,local_size_y = 30,local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D imgInput;
layout(rgba32f, binding = 1) uniform image2D imgOutput;

uniform vec4 draw_color;

float activation(float x) {
    return -1.0/pow(2.0, (0.6*pow(x, 2.0)))+1.0;
}

ivec2 find_pixel(ivec2 texelCoord, ivec2 offset) {
    ivec2 size = imageSize(imgInput);
    return ivec2(mod(texelCoord+offset,size));
}

void main() {
    ivec2 texelCoord = ivec2(gl_GlobalInvocationID.xy);
    
    float total = 0;
    float kernal[9] = {
        0.67865,-0.90,0.67865,
        -0.90,-0.66,-0.90,
        0.67865,-0.90,0.67865
    };

    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2(-1,-1))).w*kernal[8];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 0,-1))).w*kernal[7];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 1,-1))).w*kernal[6];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2(-1,0 ))).w*kernal[5];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 0,0 ))).w*kernal[4];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 1,0 ))).w*kernal[3];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2(-1,1 ))).w*kernal[2];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 0,1 ))).w*kernal[1];
    total += imageLoad(imgInput,find_pixel(texelCoord,ivec2( 1,1 ))).w*kernal[0];
    
    float val = clamp(activation(total),0.0,1.0);

    vec4 out_pixel = vec4(draw_color.xyz*val,val);

    imageStore(imgOutput,texelCoord,out_pixel);
    
}