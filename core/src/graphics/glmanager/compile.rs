use gl::types::*;
use std::{ffi::CString, ptr, str};

pub fn compile_shader(path: &str, src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            println!("While compiling {}", path);
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);

        let loc_pos = 0;
        let loc_normal = 1;
        let loc_tex = 2;
        let loc_fnormal = 3;

        let c_str = CString::new("io_pos".as_bytes()).unwrap();
        gl::BindAttribLocation(program, loc_pos, c_str.as_ptr());
        let c_str = CString::new("io_normal".as_bytes()).unwrap();
        gl::BindAttribLocation(program, loc_normal, c_str.as_ptr());
        let c_str = CString::new("io_tex".as_bytes()).unwrap();
        gl::BindAttribLocation(program, loc_tex, c_str.as_ptr());
        let c_str = CString::new("io_fnormal".as_bytes()).unwrap();
        gl::BindAttribLocation(program, loc_fnormal, c_str.as_ptr());

        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf).expect("ProgramInfoLog is not valid utf-8")
            );
        }

        let mut count = 0;
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut count);

        /*
        print list of uniforms
        for i in 0..count
        {
            let mut size = 0 as i32;
            let mut t = 0 as u32;
            let mut name = [0; 64];
            let mut length = 0;
            gl::GetActiveUniform(program, i as u32, 64, &mut length, &mut size, &mut t, name.as_mut_ptr());
            let name: Vec<u8> = name.iter().map(|&x| x as u8).collect();
            let name = std::str::from_utf8(&name).unwrap();
            println!("{} {}", i, name);
            //printf("Uniform #%d Type: %u Name: %s\n", i, type, name);
        }
        */

        program
    }
}
