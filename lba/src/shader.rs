use std::ffi::{CString};

use crate::gl;
use crate::glam;
use std;

#[inline]
fn string_to_c_str(string:String) -> CString {
    CString::new(string).unwrap()
}

fn check_shader(id:u32,status:gl::types::GLenum) { //gl::COMPILE_STATUS or gl::LINK_STATUS
    if status != gl::COMPILE_STATUS && status != gl::LINK_STATUS {
        println!("[check_shader] invalid status");
    }

    let mut success = 1;
    unsafe {gl::GetShaderiv(id, status, &mut success)};
    if success == 0 {
        let mut length = 0;
        unsafe {gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut length)};

        let mut buffer:Vec<u8> = Vec::with_capacity(length as usize+1);
        buffer.extend([b' '].iter().cycle().take(length as usize));
        let mut written:gl::types::GLsizei = 0;
        let error = unsafe {CString::from_vec_unchecked(buffer)};

        unsafe {gl::GetShaderInfoLog(id,length+1,&mut written,error.as_ptr() as *mut gl::types::GLchar)};

        println!("Error {:?}",error);
    }
}

pub struct Shader <'a> {
    filepath:&'a str,
    shadertype:gl::types::GLenum,
    id:u32
}

impl Shader<'_> {
    pub fn new(filepath:&str,shadertype:gl::types::GLenum) -> Shader{
        Shader {
            filepath,
            shadertype,
            id: 0,
        }
    }

    fn compile_shader(&mut self) {
        let mut contents = std::fs::read(self.filepath).unwrap();
        contents.push(0);
        let binding = CString::from_vec_with_nul(contents).unwrap();
        //println!("{:?}",binding);
        let str = binding.as_c_str();
    
        self.id = unsafe {gl::CreateShader(self.shadertype)};
    
        unsafe {
            gl::ShaderSource(self.id,1,&str.as_ptr(),0 as *const i32);
            gl::CompileShader(self.id);
        }
        check_shader(self.id,gl::COMPILE_STATUS);
    }
}

impl Drop for Shader <'_> {
    fn drop(&mut self) {
        unsafe {gl::DeleteShader(self.id)};
    }
}


pub struct Program {
    pub id:u32
}
impl Program {
    pub fn link_program<T, const N: usize>(&mut self,shaders:[Shader;N]) {
        self.id = unsafe {gl::CreateProgram()};
    
        for mut shader in shaders {
            shader.compile_shader();
            unsafe {gl::AttachShader(self.id,shader.id)};
        }
    
        unsafe {
            gl::LinkProgram(self.id);
            gl::ValidateProgram(self.id);
        }
    
        check_shader(self.id, gl::LINK_STATUS);
        
    }

    pub fn new<T, const N: usize>(shaders:[Shader;N]) -> Program {
        let mut pro:Program = Program {
            id:0
        };
        pro.link_program::<usize,N>(shaders);
        
        return pro;
        
    }

    pub fn bind(&self) {
        unsafe {gl::UseProgram(self.id)};
    }
    

    
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {gl::DeleteProgram(self.id)};
    }
}

pub trait Uniforms {
    fn set_bool(&self,name:&str,value:bool);
    fn set_int(&self,name:&str,value:i32);
    fn set_float(&self,name:&str,value:f32);
    fn set_vec2(&self,name:&str,value:glam::Vec2);
    fn set_vec2s(&self,name:&str,x:f32,y:f32);
    fn set_vec3(&self,name:&str,value:glam::Vec3);
    fn set_vec3s(&self,name:&str,x:f32,y:f32,z:f32);
    fn set_vec4(&self,name:&str,value:glam::Vec4);
    fn set_vec4s(&self,name:&str,x:f32,y:f32,z:f32,w:f32);
    fn set_mat2(&self,name:&str,value:glam::Mat2);
    fn set_mat3(&self,name:&str,value:glam::Mat3);
    fn set_mat4(&self,name:&str,value:glam::Mat4);
}
impl Uniforms for Program {
    fn set_bool(&self,name:&str,value:bool) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform1i(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            value as i32)};

    }

    fn set_int(&self,name:&str,value:i32) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform1i(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            value)};
    }

    fn set_float(&self,name:&str,value:f32) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform1f(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            value)};
    }

    fn set_vec2(&self,name:&str,value:glam::Vec2) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform2fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            &value[0])};
    }

    fn set_vec2s(&self,name:&str,x:f32,y:f32) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform2f(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            x,
            y,)};
    }

    fn set_vec3(&self,name:&str,value:glam::Vec3) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform3fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            &value[0])};
    }

    fn set_vec3s(&self,name:&str,x:f32,y:f32,z:f32) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform3f(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            x,
            y,
            z)};
    }

    fn set_vec4(&self,name:&str,value:glam::Vec4) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform4fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            &value[0])};
    }

    fn set_vec4s(&self,name:&str,x:f32,y:f32,z:f32,w:f32) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::Uniform4f(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            x,
            y,
            z,
            w)};
    }

    fn set_mat2(&self,name:&str,value:glam::Mat2) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::UniformMatrix2fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            gl::FALSE,
            &value.to_cols_array()[0])};
    }

    fn set_mat3(&self,name:&str,value:glam::Mat3) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::UniformMatrix3fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            gl::FALSE,
            &value.to_cols_array()[0])};
    }

    fn set_mat4(&self,name:&str,value:glam::Mat4) {
        let name = string_to_c_str(name.to_string());
        unsafe {gl::UniformMatrix4fv(gl::GetUniformLocation(
            self.id,
            name.as_ptr()),
            1,
            gl::FALSE,
            &value.to_cols_array()[0])};
    }
}




