extern crate image;
use image::GenericImageView;
use image_to_ascii::text_formats;
use std::fs;

const ASCII_CHARS: &[char] = &[' ', '!', ' ', ':', '-', '=', '+', '*', '#', '%', '@', '&'];
const UNICODE_CHARS: &[char] = &['█', '▓', '▒', '░', ' '];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_path = "resources/Sample4.jpg";
    let img = image::open(image_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let img = img.to_rgba8();

    let scale_factor = 0.1;
    let s_width = (width as f32 * scale_factor) as u32;
    let s_height = (height as f32 * scale_factor) as u32;
    let ascii_image = text_formats::convert_to_text(&img, s_width, s_height, UNICODE_CHARS);

    fs::write("output/output.txt", ascii_image)?;
    println!("Text has been written to the file.");
    Ok(())
}
