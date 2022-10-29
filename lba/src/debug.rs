use std::ffi::CStr;
use crate::gl;

pub extern "system" 
    fn message_callback(_source:gl::types::GLenum,
        types:gl::types::GLenum,
        _id:gl::types::GLuint,
        severity:gl::types::GLenum,
        _length:gl::types::GLsizei,
        message:*const gl::types::GLchar,
        _user_param:*mut gl::types::GLvoid)
     {
         let gl_error:&str;
         if types == gl::DEBUG_TYPE_ERROR {
             gl_error = "** GL ERROR **";
         } else {
             gl_error = "";
         }
         let msg = unsafe {CStr::from_ptr(message)};
         println!("GL CALLBACK: {} type = 0x{:?}, severity = 0x{:?}, message = {:?}\n",gl_error,types,severity,msg);  
     }
//example use case 
// unsafe {
//     gl::Enable(gl::DEBUG_OUTPUT);
//     gl::DebugMessageCallback(std::option::Option::Some(message_callback),0 as *const gl::types::GLvoid);
// }


pub fn get_errors() {
    let mut err:gl::types::GLenum;
    unsafe {
        err = gl::GetError();
        while err != gl::NO_ERROR
        {
            println!("Error  {:?} ",err);
            err = gl::GetError();
        }

    }
}