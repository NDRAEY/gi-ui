#[cfg(not(feature = "no_std"))]
use std::fs::File;

use crate::canvas::Canvas;

#[cfg(feature = "png")]
pub fn export_to_png(canvas: &Canvas) {
    let width = canvas.width();
    let height = canvas.height();
    let buffer: &[u8] = canvas.buffer();

    {
        if std::fs::exists("./out.png").unwrap() {
            std::fs::remove_file("./out.png").unwrap();
        }

        let file = File::create("./out.png").unwrap();
        let writer = std::io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(buffer).unwrap();
    }
}
