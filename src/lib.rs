pub mod text_formats {
    use image::ImageBuffer;
    use image::Rgba;

    pub fn convert_to_text(
        img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        scaled_width: u32,
        scaled_height: u32,
        chars: &[char],
    ) -> String {
        let mut image_text = String::new();

        for y in 0..scaled_height {
            for x in 0..scaled_width {
                let pixel_x = (x as f32 / scaled_width as f32 * img.width() as f32) as u32;
                let pixel_y = (y as f32 / scaled_height as f32 * img.height() as f32) as u32;

                let pixel = img.get_pixel(pixel_x, pixel_y);
                let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0;
                let index =
                    chars.len() - 1 - (intensity / 255.0 * (chars.len() - 1) as f32) as usize;
                image_text.push(' ');
                image_text.push(chars[index]);
            }
            image_text.push('\n');
        }
        image_text
    }
}
