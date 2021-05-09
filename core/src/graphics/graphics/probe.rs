use super::*;
use fere_common::*;

impl Graphics {
    /// Does some shit
    ///
    /// # Safety
    /// TODO
    pub unsafe fn probe_read_depth(&self, buffer: *mut f32) {
        let size = self.pass_probe.outputs_get()[0].size_get();
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.pass_probe.raw_get());
        gl::ReadPixels(
            0,
            0,
            size.x,
            size.y,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            buffer.cast(),
        );
    }

    /// Does some shit
    ///
    /// # Safety
    /// TODO
    pub unsafe fn probe_read_diffuse(&self, buffer: *mut Vec3) {
        let size = self.pass_probe.outputs_get()[0].size_get();
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.pass_probe.raw_get());
        gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
        gl::ReadPixels(0, 0, size.x, size.y, gl::RGB, gl::FLOAT, buffer.cast());
    }

    /// Does some shit
    ///
    /// # Safety
    /// TODO
    pub unsafe fn probe_read_illumination(&self, buffer: *mut Vec3) {
        let size = self.pass_probe.outputs_get()[0].size_get();
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.pass_probe.raw_get());
        gl::ReadBuffer(gl::COLOR_ATTACHMENT1);
        gl::ReadPixels(0, 0, size.x, size.y, gl::RGB, gl::FLOAT, buffer.cast());
    }
}
