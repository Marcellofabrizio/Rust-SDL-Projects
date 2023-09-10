use image::bmp::BmpEncoder;
use image::{ImageBuffer, Pixel, Rgba};
use std::fs::File;
use std::io::BufWriter;

use sdl2::render::{Canvas, Texture};
use sdl2::video::{Window, WindowContext};

pub fn save_canvas_to_bmp(
    texture: &mut Texture,
    canvas: &mut Canvas<Window>,
) -> Result<(), image::ImageError> {
    let query = texture.query();

    let width = query.width;
    let height = query.height;

    let mut canvas_pixels = canvas
        .read_pixels(None, sdl2::pixels::PixelFormatEnum::ARGB8888)
        .expect("Read pixels failes");

    let mut image_buffer = ImageBuffer::<Rgba<u8>, _>::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = ((y * width + x) * 4) as usize;
            let pixel = Rgba([
                canvas_pixels[index + 2],
                canvas_pixels[index + 1],
                canvas_pixels[index],
                canvas_pixels[index + 3],
            ]);
            image_buffer.put_pixel(x, y, pixel);
        }
    }

    // Create a BMP encoder and save the image to a file
    let output_file = File::create("output.bmp").expect("Failed to create BMP file");
    let mut buf_writer = BufWriter::new(output_file);
    let mut encoder = BmpEncoder::new(&mut buf_writer);

    encoder
        .encode(
            &image_buffer,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )
        .expect("Failed to encode BMP image");

    println!("Image saved as 'output.bmp'");

    Ok(())
}
