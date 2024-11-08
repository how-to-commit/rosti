use bootloader_api::info::{FrameBufferInfo, PixelFormat};

pub struct FrameBufferWriter {
    buf: &'static mut [u8],
    info: FrameBufferInfo,
    pos_x: usize,
    pos_y: usize,
}

impl FrameBufferWriter {
    pub fn new(buf: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self {
            buf,
            info,
            pos_x: 0,
            pos_y: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.info.width
    }

    pub fn height(&self) -> usize {
        self.info.height
    }

    pub fn pos(&self) -> [usize; 2] {
        [self.pos_x, self.pos_y]
    }

    /// Draws a given pixel to the framebuffer.
    pub fn draw_pixel(&mut self, pos_x: usize, pos_y: usize, red: u8, green: u8, blue: u8) {
        let pixel_offset = pos_y * self.info.stride + pos_x;
        let pixel_colour = match self.info.pixel_format {
            PixelFormat::Rgb => [red, green, blue],
            PixelFormat::Bgr => [blue, green, red],
            PixelFormat::U8 => {
                // convert rgb to grayscale
                let sum: u32 = 30 * red as u32 + 59 * green as u32 + 11 * blue as u32;
                let grayscale_col: u8 = u8::try_from(sum / 10).unwrap();
                [grayscale_col, 0, 0]
            }
            other => {
                panic!("Pixel format {:?} not supported!", other);
            }
        };

        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;

        self.buf[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&pixel_colour[..bytes_per_pixel]);
    }
}
