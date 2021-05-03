use crate::glmanager::shader::Shader;
use std::sync::Arc;

pub struct Programs {
    pub dr_1: Arc<Shader>,
    pub dr_2: Arc<Shader>,
    pub dr_2_ambient: Arc<Shader>,
    pub dr_2_irradiance: Arc<Shader>,
    pub dr_2_omni: Arc<Shader>,
    pub dr_3: Arc<Shader>,

    pub debug_test: Arc<Shader>,
    pub debug_iden: Arc<Shader>,
    pub debug_depth: Arc<Shader>,

    pub basic: Arc<Shader>,
    pub standard: Arc<Shader>,
    pub standard_probe: Arc<Shader>,

    pub shadow_1: Arc<Shader>,
    pub probe: Arc<Shader>,

    pub sh_visualize: Arc<Shader>,
    pub sh_visualize_single: Arc<Shader>,

    pub geo_visualize: Arc<Shader>,

    pub image: Arc<Shader>,
}

impl Programs {
    pub fn new(glmanager: &crate::glmanager::GlManager) -> Self {
        Programs {
            dr_1: glmanager.get_program("dr_1"),
            dr_2: glmanager.get_program("dr_2"),
            dr_2_ambient: glmanager.get_program("dr_2_ambient"),
            dr_2_irradiance: glmanager.get_program("dr_2_irradiance"),
            dr_2_omni: glmanager.get_program("dr_2_omni"),
            dr_3: glmanager.get_program("dr_3"),

            debug_test: glmanager.get_program("debug_test"),
            debug_iden: glmanager.get_program("debug_iden"),
            debug_depth: glmanager.get_program("debug_depth"),

            basic: glmanager.get_program("basic"),
            standard: glmanager.get_program("standard"),
            standard_probe: glmanager.get_program("standard_probe"),

            shadow_1: glmanager.get_program("shadow_1"),
            probe: glmanager.get_program("probe"),

            sh_visualize: glmanager.get_program("sh_visualize"),
            sh_visualize_single: glmanager.get_program("sh_visualize_single"),

            geo_visualize: glmanager.get_program("geo_visualize"),

            image: glmanager.get_program("image"),
        }
    }
}
