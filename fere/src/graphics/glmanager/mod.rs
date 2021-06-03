mod compile;
pub mod light;
pub mod shader;

use super::*;
use shader::*;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::{collections::HashMap, io::prelude::*, sync::Arc};

fn get_shader_config_path() -> PathBuf {
    let mut path_to_shader = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_to_shader.push("shaders/config.yaml");
    path_to_shader
}

fn get_shader_path(path: &str) -> PathBuf {
    let mut path_to_shader = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_to_shader.push(format!("shaders/{}", path));
    path_to_shader
}
pub struct GlManager {
    programs: HashMap<String, Arc<Shader>>,
}

impl GlManager {
    pub fn new(_name: String) -> Self {
        unsafe {
            let ver = std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION).cast());
            println!("GL VERSION: {:?}", ver);
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            #[cfg(not(target_os = "macos"))]
            {
                gl::DebugMessageCallback(Some(error_handler), std::ptr::null());
            }
        }

        let programs_: HashMap<String, (String, String)> =
            serde_yaml::from_str(&read_to_string(get_shader_config_path()).unwrap()).unwrap();

        let mut programs = HashMap::new();

        for (name, (vert, frag)) in programs_ {
            let vert = get_shader_path(&vert);
            let frag = get_shader_path(&frag);
            programs.insert(
                name.clone(),
                Arc::new(Shader::new(
                    name,
                    &read_to_string(vert.clone()).unwrap(),
                    vert.to_str().unwrap(),
                    &read_to_string(frag.clone()).unwrap(),
                    frag.to_str().unwrap(),
                )),
            );
        }
        GlManager { programs }
    }

    pub fn get_program(&self, key: &str) -> Arc<Shader> {
        let program = self.programs.get(key).unwrap();
        Arc::clone(&program)
    }
}

/// When OpenGL meets an error, this callback function will be called on the same stack.
#[cfg(not(target_os = "macos"))]
extern "system" fn error_handler(
    source: u32,
    the_type: u32,
    id: u32,
    severity: u32,
    _length: i32,
    message: *const i8,
    _userparma: *mut std::ffi::c_void,
) {
    if the_type != gl::DEBUG_TYPE_ERROR {
        return;
    }
    unsafe {
        let msg = std::ffi::CString::from_raw(message as *mut std::os::raw::c_char)
            .into_string()
            .unwrap();
        panic!(
            "OpenGL Error: Source: {:x} / Type: {:x} / Id: {:x} / Severity: {:x} => {}",
            source, the_type, id, severity, msg
        );
    }
}
