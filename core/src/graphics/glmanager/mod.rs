mod compile;
pub mod light;
pub mod shader;

use shader::*;
use std::{collections::hash_map::HashMap, io::prelude::*, sync::Arc};
use tpf_package::static_map::{StaticMap, StaticMapKind};

pub struct StaticMapShader;
impl StaticMapKind for StaticMapShader {
    fn kind() -> i16 {
        100
    }
}

pub struct GlManager {
    programs: StaticMap<Arc<Shader>, StaticMapShader>,
}

const ERROR_MSG: &str = "Failed to read shaders. Fatal Error";

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

        let programs_list: Vec<(String, String, String)> =
            tpf_package::read_yaml("$C/shader/config.yaml").expect(ERROR_MSG);

        let mut programs = HashMap::new();

        for (name, vert, frag) in programs_list {
            programs.insert(
                name.clone(),
                Arc::new(Shader::new(
                    name,
                    &{
                        let mut buf = String::new();
                        tpf_package::read_file(&vert)
                            .expect(ERROR_MSG)
                            .read_to_string(&mut buf)
                            .expect(ERROR_MSG);
                        buf
                    },
                    &vert,
                    &{
                        let mut buf = String::new();
                        tpf_package::read_file(&frag)
                            .expect(ERROR_MSG)
                            .read_to_string(&mut buf)
                            .expect(ERROR_MSG);
                        buf
                    },
                    &frag,
                )),
            );
        }
        let programs = StaticMap::new(programs);
        GlManager { programs }
    }

    pub fn get_program(&self, key: &str) -> Arc<Shader> {
        let program = self.programs.get(key);
        program.clone()
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
