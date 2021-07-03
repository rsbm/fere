use fere_common::*;
use std::net::{TcpListener, TcpStream};

pub struct VideoRecordingSession {
    frame_recorded: usize,
    encoder: y4m::Encoder<TcpStream>,

    buffer_y: Vec<u8>,
    buffer_cb: Vec<u8>,
    buffer_cr: Vec<u8>,
}

impl VideoRecordingSession {
    pub fn new(port: u16, size: IVec2, fps: usize) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", 5555)).unwrap();

        println!("Waiting for a single TCP connection at port {}..", port);
        println!("The program will be blocked until you connect FFMpeg here");
        println!("Use -i tcp://127.0.0.1:{}", port);
        let (socket, addr) = listener.accept().unwrap();
        print!("Succeeded to accept a client: {:?}", addr);

        let encoder = y4m::encode(size.x as usize, size.y as usize, y4m::Ratio::new(fps, 1))
            .with_colorspace(y4m::Colorspace::C444)
            .write_header(socket)
            .unwrap();

        let buffer_size = size.x as usize * size.y as usize;
        Self {
            frame_recorded: 0,
            encoder,
            buffer_y: vec![0; buffer_size],
            buffer_cb: vec![0; buffer_size],
            buffer_cr: vec![0; buffer_size],
        }
    }

    /// It reads data from color attachments (0: y, 1: cb, 2: cr)
    pub fn update_frame(&mut self, graphics: &crate::Graphics) {
        unsafe {
            graphics.read_yuv(
                self.buffer_y.as_mut_ptr(),
                self.buffer_cb.as_mut_ptr(),
                self.buffer_cr.as_mut_ptr(),
                self.buffer_y.len(),
            );
        }

        let frame_out = y4m::Frame::new(
            [
                self.buffer_y.as_slice(),
                self.buffer_cb.as_slice(),
                self.buffer_cr.as_slice(),
            ],
            None,
        );
        self.encoder.write_frame(&frame_out).unwrap();
        self.frame_recorded += 1;
    }

    /// Returns the session result in a text message.
    pub fn end(self) {
        drop(self.encoder);
        println!("Video recording successfully finished");
    }
}
