use image::DynamicImage;
use image::GenericImageView;
use minifb::{Window, WindowOptions};
use std::time::{Duration, Instant};

pub fn show_image(
    img: &DynamicImage,
    title: &str,
    zoom: f64,
    time_secs: u64,
) -> Result<(), String> {
    
    let (img_x, img_y) = img.dimensions();

    // create buffer
    let buffer: Vec<u32> = img
        .pixels()
        .map(|(_, _, rgba)| {
            let r = rgba[0] as u32;
            let g = rgba[1] as u32;
            let b = rgba[2] as u32;
            (r << 16) | (g << 8) | b
        })
        .collect();

    // create window
    let window_x = (img_x as f64 * zoom) as usize;
    let window_y = (img_y as f64 * zoom) as usize;

    let mut window = Window::new(title, window_x, window_y, WindowOptions::default())
        .map_err(|e| format!("failed to make window: {}", e))?;

    // start timer
    let start = Instant::now();
    let interval = Duration::from_secs(time_secs);

    // window loop
    while window.is_open() && start.elapsed() < interval {
        window
            .update_with_buffer(&buffer, img_x as usize, img_y as usize)
            .map_err(|e| format!("failed to load buffer: {}", e))?;
    }

    Ok(())
}
