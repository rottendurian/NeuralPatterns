use lba::{shader::{Program, Shader}, gl, camera::Camera2D, glam};
use rand;
use rand::prelude::*;
use std::{mem::size_of};
use lba::shader::Uniforms;

const TEXTUREWIDTH:i32 =  3000;
const TEXTUREHEIGHT:i32 = 3000;

pub struct Render {
    vao:gl::types::GLuint,
    vbo:gl::types::GLuint,
    texture:gl::types::GLuint,
    texture2:gl::types::GLuint,
    compute:Program,
    pub compute_interval:f64,
    compute_swap:bool,
    square:Program,
    draw:Program,
    draw_size:u32,
    pub color:glam::Vec4
}

impl Render {
    pub fn new(compute:&str,vertex:&str,fragment:&str,color:glam::Vec4) -> Render {
        
            let ret = Render {
            vao:0,
            vbo:0,
            texture:0,
            texture2:0,
            compute:Program::new::<usize,1>(
                [Shader::new(compute,gl::COMPUTE_SHADER)]),
            compute_interval:0.1,
            compute_swap:true,
            square:Program::new::<usize,2>(
                [Shader::new(vertex,gl::VERTEX_SHADER),
                         Shader::new(fragment,gl::FRAGMENT_SHADER)]),
            draw:Program::new::<usize,1>([Shader::new("res/shaders/buffered_draw.shader",gl::COMPUTE_SHADER)]),
            draw_size:50,
            color
        };

        return ret;
    }
    pub fn change_compute_shader(&mut self, file:&str) {
        self.compute = Program::new::<usize,1>([Shader::new(file,gl::COMPUTE_SHADER)]);
    }

    pub fn gen_texture(&mut self) {
        let mut rng = rand::thread_rng();
        let mut vector:Vec<f32> = vec![0.0;TEXTUREWIDTH as usize * TEXTUREHEIGHT as usize*4];
        let mut iter:usize = 0;
        while iter < TEXTUREWIDTH as usize*TEXTUREHEIGHT as usize*4 {

            let num:f32 = rng.gen();
            if num > 0.6 {
                vector[iter+2] = 1.0;
                vector[iter+3] = 1.0;
            }
            
            iter+=4;
        }
        //rng.fill(&mut vector[..]);

        //println!("{:?}",vector);

        unsafe {
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.texture);
            gl::TextureParameteri(self.texture,gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(self.texture,gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(self.texture,gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(self.texture,gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl::TextureStorage2D(self.texture, 1, gl::RGBA32F, TEXTUREWIDTH, TEXTUREHEIGHT);
            gl::TextureSubImage2D(self.texture,0,0,0,TEXTUREWIDTH,TEXTUREHEIGHT,gl::RGBA,gl::FLOAT,vector.as_ptr() as *const gl::types::GLvoid);
            //gl::GenerateTextureMipmap(self.texture);
            
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut self.texture2);
            gl::TextureParameteri(self.texture2,gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(self.texture2,gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TextureParameteri(self.texture2,gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(self.texture2,gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureStorage2D( self.texture2, 1, gl::RGBA32F, TEXTUREWIDTH, TEXTUREHEIGHT);

        }
        self.compute_swap = false;
    }
    pub fn reset_texture(&mut self) {
        let mut rng = rand::thread_rng();
        let mut vector:Vec<f32> = vec![0.0;TEXTUREWIDTH as usize * TEXTUREHEIGHT as usize*4];
        let mut iter:usize = 0;
        while iter < TEXTUREWIDTH as usize*TEXTUREHEIGHT as usize*4 {

            let num:f32 = rng.gen();
            if num > 0.6 {
                vector[iter+2] = 1.0;
                vector[iter+3] = 1.0;
            }
            
            iter+=4;
        }
        unsafe{
            gl::TextureSubImage2D(self.texture,0,0,0,TEXTUREWIDTH,TEXTUREHEIGHT,gl::RGBA,gl::FLOAT,vector.as_ptr() as *const gl::types::GLvoid);
            //gl::TextureSubImage2D(self.texture2,0,0,0,TEXTUREWIDTH,TEXTUREHEIGHT,gl::RGBA,gl::FLOAT,0 as *const gl::types::GLvoid);
        }
        self.compute_swap = false;
    }

    pub fn bind_texture(&mut self) {
        if self.compute_swap == true {
            unsafe {
                gl::BindImageTexture(0,self.texture,0,gl::FALSE,0,gl::READ_ONLY,gl::RGBA32F);
                gl::BindTextureUnit(0,self.texture);
                gl::BindImageTexture(1,self.texture2,0,gl::FALSE,0,gl::WRITE_ONLY,gl::RGBA32F);
                gl::BindTextureUnit(1,self.texture2);        
            }
        }
        else {
            unsafe {
                gl::BindImageTexture(1,self.texture,0,gl::FALSE,0,gl::WRITE_ONLY,gl::RGBA32F);
                gl::BindTextureUnit(1,self.texture);
                gl::BindImageTexture(0,self.texture2,0,gl::FALSE,0,gl::READ_ONLY,gl::RGBA32F);
                gl::BindTextureUnit(0,self.texture2);        
            }
        }
        self.compute_swap = !self.compute_swap;
    }
    pub fn change_draw_size(&mut self, increase:bool) {
        if increase == true {
            self.draw_size+=1;
        }
        else {
            self.draw_size-=1;
        }
        println!("Draw size: {}",self.draw_size);
    }
    pub fn point_texture(&mut self, x:f32,y:f32,remove:bool) {
       
        self.draw.bind();
        self.draw.set_vec2("draw",glam::vec2(x*TEXTUREWIDTH as f32,y*TEXTUREWIDTH as f32));
        if remove == true {
            self.draw.set_vec4("draw_color",glam::vec4(0.0,0.0,0.0,0.0));
        } else {
            self.draw.set_vec4("draw_color",self.color);
        }

        unsafe{gl::DispatchCompute(self.draw_size,self.draw_size, 1 as u32);}

        unsafe{gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT)};
        
    }

    pub fn run_compute(&mut self) {
        self.bind_texture();
        self.compute.bind();
        self.compute.set_vec4("draw_color",self.color);
        //self.bind_texture();
        unsafe {
            gl::DispatchCompute(TEXTUREWIDTH as u32/30,TEXTUREHEIGHT as u32/30,1);
            gl::MemoryBarrier(gl::SHADER_IMAGE_LOAD | gl::SHADER_IMAGE_STORE | gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
            
            
            
        }

    }

    pub fn gen_square(&mut self) {
        // let array:Vec<f32> = vec![  
        //      0.0,  1.0,  0.0, 0.5, 1.0,
		// 	 0.0,  0.0,  0.0, 0.5, 0.5,
		// 	 1.0,  1.0,  0.0, 1.0, 1.0,
		// 	 1.0,  0.0,  0.0, 1.0, 0.5,
        // ];
        
        let array:Vec<f32> = vec![
            -1.0,  1.0, -1.0, 0.0, 1.0,
			-1.0, -1.0, -1.0, 0.0, 0.0,
			 1.0,  1.0, -1.0, 1.0, 1.0,
			 1.0, -1.0, -1.0, 1.0, 0.0,
        ];
        unsafe {
            gl::CreateVertexArrays(1,&mut self.vao);
            gl::CreateBuffers(1,&mut self.vbo);
            
            gl::NamedBufferData(self.vbo,(array.len() * size_of::<f32>()) as gl::types::GLsizeiptr,
            array.as_ptr() as *const gl::types::GLvoid,gl::STATIC_DRAW);        

            gl::EnableVertexArrayAttrib(self.vao,0);
            gl::VertexArrayAttribBinding(self.vao,0,0);
            gl::VertexArrayAttribFormat(self.vao,0,3,gl::FLOAT,gl::FALSE,0);
            
            gl::EnableVertexArrayAttrib(self.vao,1);
            gl::VertexArrayAttribBinding(self.vao,1,0);
            gl::VertexArrayAttribFormat(self.vao,1,2,gl::FLOAT,gl::FALSE,3*size_of::<f32>() as u32);
            
            gl::VertexArrayVertexBuffer(self.vao,0,self.vbo,0,5*size_of::<f32>() as i32);
        }
    }

    pub fn draw_square(&self,camera:&Camera2D) {
        unsafe {gl::BindVertexArray(self.vao);}
        self.square.bind();
        self.square.set_int("tex", 1);
        self.square.set_mat4("proj",camera.proj);
        self.square.set_mat4("view",camera.view);
        self.square.set_mat4("model",camera.model);

        unsafe {gl::DrawArrays(gl::TRIANGLE_STRIP,0,4)};
        
    }

}

impl Drop for Render {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1,&self.vbo);
            gl::DeleteTextures(1,&self.texture);
        }
    }
}
