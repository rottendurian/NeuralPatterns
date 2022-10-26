#version 460 core

layout (local_size_x = 30,local_size_y = 30,local_size_z = 1) in;

layout(rgba32f, binding = 0) uniform image2D imgInput;
layout(rgba32f, binding = 1) uniform image2D imgOutput;

uniform vec4 draw_color;

//uniform float kernal[9];

float activation(float x) {
  if (x == 3. || x == 11. || x == 12.){
    return 1.;
  }
  return 0.;
}
ivec2 find_pixel(ivec2 texelCoord, ivec2 offset) {
    ivec2 size = imageSize(imgInput);
    return ivec2(mod(texelCoord+offset,size));
}
//return ivec2(mod(texelCoord.x+offset.x,2400),mod(texelCoord.y+offset.y,2400));
// float activation(float x) {
//     return -1.0/pow(2.0, (0.6*pow(x, 2.0)))+1.0;
// }

void main() {
    ivec2 texelCoord = ivec2(gl_GlobalInvocationID.xy);
    
    float total = 0;
    float kernal[9] = {
        1.0,1.0,1.0,
        1.0,9.0,1.0,
        1.0,1.0,1.0
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
    
    float val = activation(total);

    vec4 out_pixel = vec4(draw_color.xyz*val,val);

    imageStore(imgOutput,texelCoord,out_pixel);
    
}


    // vec3 filter_1 = vec3( 0.68,-0.90, 0.68);
    // vec3 filter_2 = vec3(-0.90,-0.66,-0.90);
    // vec3 filter_3 = vec3( 0.68,-0.90, 0.68);

    // vec4 temp = imageLoad(imgOutput,texelCoord+ivec2(-1,-1));
    // average+=temp.z*filter_1[0];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(-1,0));
    // average+=temp.z*filter_1[1];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(-1,1));
    // average+=temp.z*filter_1[2];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(0,-1));
    // average+=temp.z*filter_2[0];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(0,0));
    // average+=temp.z*filter_2[1];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(0,1));
    // average+=temp.z*filter_2[2];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(1,-1));
    // average+=temp.z*filter_3[0];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(1,0));
    // average+=temp.z*filter_3[1];
    // count+=1;
    // temp = imageLoad(imgOutput,texelCoord+ivec2(1,1));
    // average+=temp.z*filter_3[2];
    // count+=1;

       // int i = -1;
    // int j = -1;
    // while(i <= 1) {
    //     while( j <= 1) {
    //         vec4 temp = imageLoad(imgOutput,texelCoord+ivec2(i,j));
    //         //if (temp.z >= 0.5) {
    //         //    count+=1;
    //         //}
    //         if (!(i==0 && j==0)) {
    //             average+=temp.z;
    //             count+=1.0;
    //         }
    //         j++;
    //     }
    //     j=-1;
    //     i+=1;
    // }


    // float outer  =   1.0;
    // float inner  =   1.0;
    // float middle =   9.0;
    



    






    // vec4 cur = imageLoad(imgOutput,texelCoord+ivec2(-1,-1));
    // average+=cur.z*outer;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(-1,0));
    // average+=cur.z*inner;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(-1,1));
    // average+=cur.z*outer;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(0,-1));
    // average+=cur.z*inner;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(0,0));
    // average+=cur.z*middle;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(0,1));
    // average+=cur.z*inner;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(1,-1));
    // average+=cur.z*outer;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(1,0));
    // average+=cur.z*inner;
    // //count+=1;
    // cur = imageLoad(imgOutput,texelCoord+ivec2(1,1));
    // average+=cur.z*outer;
    // //count+=1;


 


    // vec4 value = imageLoad(imgOutput,texelCoord);

    // float act = activation(average);

    // value = vec4(0.0,0.0,act,0.0);
   
    // //value = clamp(value,vec4(-1.0),vec4(1.0));


    // // //value += vec4(0.0001,0.0,0.0,0.0);
    // // if (value.z >= 0.5) {
    // //     count--;
    // //     if(!(count == 3 || count == 2)) {
    // //         value = vec4(0.0);
    // //     }
    // // } 
    // // else {
    // //     if (count == 3) {
    // //         value = vec4(0.0,0.5,0.5,1.0);
    // //     }
    // // }

    

    // imageStore(imgOutput, texelCoord, value);