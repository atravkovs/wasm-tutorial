use base64::{write::EncoderStringWriter, STANDARD};
use image::{self, DynamicImage, ImageBuffer, ImageOutputFormat, ImageResult, Rgb};
use num_complex::Complex;

fn base64_png(img: DynamicImage) -> ImageResult<String> {
    let mut buf = String::from("");

    {
        let mut writer = EncoderStringWriter::from(&mut buf, STANDARD);
        img.write_to(&mut writer, ImageOutputFormat::Png)?;
    }

    Ok(buf)
}

fn generate_julia_set(
    width: u32,
    height: u32,
    real: f32,
    imaginary: f32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let scalex = 3.0 / width as f32;
    let scaley = 3.0 / height as f32;

    let c = Complex::new(real, imaginary);

    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;

        let cx = y as f32 * scalex - 1.5;
        let cy = x as f32 * scaley - 1.5;
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }

        *pixel = Rgb([r, i as u8, b]);
    }

    imgbuf
}

pub fn julia_base64(width: u32, height: u32, real: f32, imaginary: f32) -> String {
    let image = generate_julia_set(width, height, real, imaginary);

    let res = base64_png(DynamicImage::ImageRgb8(image))
        .expect("Unable to convert image to base64 string");

    return res;
}

fn main() {
    julia_base64(800, 800, 0.4, -0.8);
}
