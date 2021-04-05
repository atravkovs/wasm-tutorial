use base64::{write::EncoderStringWriter, STANDARD};
use image::{self, DynamicImage, ImageBuffer, ImageOutputFormat, ImageResult, Rgb};
use num_complex::Complex;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn julia_base64(width: u32, height: u32, real: f32, imaginary: f32) -> JsValue {
    let image = generate_julia_set(width, height, real, imaginary);

    let base64 = base64_png(DynamicImage::ImageRgb8(image))
        .expect("Unable to convert image to base64 string");

    return JsValue::from_str(&base64);
}

#[wasm_bindgen(module = "/src/progress.js")]
extern "C" {
    pub fn update_progress(percentage: u32);
}

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
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    let red_scale = 255.0 / width as f32;
    let blue_scale = 255.0 / height as f32;

    let c = Complex::new(real, imaginary);

    let total = width * height;
    let mut image_buffer = ImageBuffer::new(width, height);

    let mut last_percentage = 0;
    let mut processed = 0;
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let r = (red_scale * x as f32) as u8;
        let b = (blue_scale * y as f32) as u8;

        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }

        processed += 1;
        let percentage = processed * 100 / total;
        if percentage > last_percentage {
            last_percentage = percentage;
            update_progress(percentage);
        }

        *pixel = Rgb([r, i as u8, b]);
    }

    image_buffer
}
