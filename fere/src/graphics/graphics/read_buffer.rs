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

    pub unsafe fn read_yuv(&self, buffer_y: *mut u8, buffer_cb: *mut u8, buffer_cr: *mut u8) {
        let pass_yuv = self.pass_yuv.as_ref().unwrap();
        let size = pass_yuv.outputs_get()[0].size_get();
        gl::BindFramebuffer(gl::FRAMEBUFFER, pass_yuv.raw_get());
        gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
        gl::ReadPixels(
            0,
            0,
            size.x,
            size.y,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer_y.cast(),
        );
        gl::ReadBuffer(gl::COLOR_ATTACHMENT1);
        gl::ReadPixels(
            0,
            0,
            size.x,
            size.y,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer_cb.cast(),
        );
        gl::ReadBuffer(gl::COLOR_ATTACHMENT2);
        gl::ReadPixels(
            0,
            0,
            size.x,
            size.y,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer_cr.cast(),
        );
    }
}
