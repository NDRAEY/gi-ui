//! Canvas is a framebuffer for rendering widgets on it.
//! Pixel format is RGBA.
pub const BITS_PER_PIXEL: usize = 32;

pub struct Canvas {
    framebuffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height * (BITS_PER_PIXEL >> 3);

        Canvas {
            framebuffer: vec![0; buffer_size],
            width,
            height,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        self.framebuffer.as_ref()
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.framebuffer.as_mut()
    }

    pub fn buffer_size(&self) -> usize {
        self.width * self.height * (BITS_PER_PIXEL >> 3)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn pixel_position(&self, x: usize, y: usize) -> usize {
        (x + (y * self.width)) * (BITS_PER_PIXEL >> 3)
    }

    #[inline]
    pub fn into_rgba(color: u32) -> (u8, u8, u8, u8) {
        let b = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let r = ((color >> 16) & 0xff) as u8;
        let a = ((color >> 24) & 0xff) as u8;

        (r, g, b, a)
    }

    /// Sets an individual pixel on the canvas
    /// Color format: `0xAARRGGBB`
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) -> Option<()> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let position = self.pixel_position(x, y);
        let (r, g, b, a) = Self::into_rgba(color);

        self.framebuffer[position + 0] = r;
        self.framebuffer[position + 1] = g;
        self.framebuffer[position + 2] = b;
        self.framebuffer[position + 3] = a;

        Some(())
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let position = self.pixel_position(x, y);

        let color = (self.framebuffer[position] as u32)
            | ((self.framebuffer[position + 1] as u32) << 8)
            | ((self.framebuffer[position + 2] as u32) << 16)
            | ((self.framebuffer[position + 3] as u32) << 24);

        Some(color)
    }
}
