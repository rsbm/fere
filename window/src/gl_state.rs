#[derive(Default)]
pub struct GlState {
    program: i32,
    framebuffer: i32,
    active_texture: i32,
    texture: i32,
    array_buffer: i32,
    vertex_array: i32,
    viewport: [i32; 4],
    scissor_box: [i32; 4],
    blend_src_rgb: i32,
    blend_dst_rgb: i32,
    blend_src_alpha: i32,
    blend_dst_alpha: i32,
    blend_eq_rgb: i32,
    blend_eq_alpha: i32,
    enable_blend: u8,
    enable_cull: u8,
    enable_depth: u8,
    enable_scissor: u8,
    enable_multisample: u8,
}

impl GlState {
    pub fn backup(&mut self) {
        unsafe {
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut self.program);
            gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut self.framebuffer);
            gl::GetIntegerv(gl::ACTIVE_TEXTURE, &mut self.active_texture);
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut self.texture);
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut self.array_buffer);
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut self.vertex_array);
            gl::GetIntegerv(gl::VIEWPORT, self.viewport.as_mut_ptr());
            gl::GetIntegerv(gl::SCISSOR_BOX, self.scissor_box.as_mut_ptr());
            gl::GetIntegerv(gl::BLEND_SRC_RGB, &mut self.blend_src_rgb);
            gl::GetIntegerv(gl::BLEND_DST_RGB, &mut self.blend_dst_rgb);
            gl::GetIntegerv(gl::BLEND_SRC_ALPHA, &mut self.blend_src_alpha);
            gl::GetIntegerv(gl::BLEND_DST_ALPHA, &mut self.blend_dst_alpha);
            gl::GetIntegerv(gl::BLEND_EQUATION_RGB, &mut self.blend_eq_rgb);
            gl::GetIntegerv(gl::BLEND_EQUATION_ALPHA, &mut self.blend_eq_alpha);
            self.enable_blend = gl::IsEnabled(gl::BLEND);
            self.enable_cull = gl::IsEnabled(gl::CULL_FACE);
            self.enable_depth = gl::IsEnabled(gl::DEPTH_TEST);
            self.enable_scissor = gl::IsEnabled(gl::SCISSOR_TEST);
            self.enable_multisample = gl::IsEnabled(gl::MULTISAMPLE);
        }
    }

    pub fn load(&self) {
        unsafe {
            gl::UseProgram(self.program as u32);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer as u32);
            gl::ActiveTexture(self.active_texture as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.texture as u32);
            gl::BindVertexArray(self.vertex_array as u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.array_buffer as u32);
            gl::BlendEquationSeparate(self.blend_eq_rgb as u32, self.blend_eq_alpha as u32);
            gl::BlendFuncSeparate(
                self.blend_src_rgb as u32,
                self.blend_dst_rgb as u32,
                self.blend_src_alpha as u32,
                self.blend_dst_alpha as u32,
            );

            if self.enable_blend == 1 {
                gl::Enable(gl::BLEND);
            } else {
                gl::Disable(gl::BLEND);
            }
            if self.enable_cull == 1 {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }
            if self.enable_depth == 1 {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
            if self.enable_scissor == 1 {
                gl::Enable(gl::SCISSOR_TEST);
            } else {
                gl::Disable(gl::SCISSOR_TEST);
            }
            if self.enable_multisample == 1 {
                gl::Enable(gl::MULTISAMPLE);
            } else {
                gl::Disable(gl::MULTISAMPLE);
            }

            gl::Viewport(
                self.viewport[0],
                self.viewport[1],
                self.viewport[2],
                self.viewport[3],
            );
            gl::Scissor(
                self.scissor_box[0],
                self.scissor_box[1],
                self.scissor_box[2],
                self.scissor_box[3],
            );
        }
    }
}
