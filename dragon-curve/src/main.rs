use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn to_bgra(color: (f64, f64, f64)) -> u32 {
    let r: u32 = (color.0 * 255.99) as u32;
    let g: u32 = (color.1 * 255.99) as u32;
    let b: u32 = (color.2 * 255.99) as u32;
    255 << 24 | r << 16 | g << 8 | b
}

fn update_framebuffer(framebuffer: &mut Vec<u32>, width: usize, height: usize, t: f64) -> () {
    for y in 0..height {
        for x in 0..width {
            // just do a gradient
            let color = (y as f64 / height as f64, x as f64 / width as f64, 0 as f64);
            let yt = (y + ((t * height as f64) as usize)) % height;
            framebuffer[yt * width + x] = to_bgra(color);
        }
    }
}

fn main() {
    println!("Creating a window...");
    let mut window = Window::new(
        "hello! - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    println!("Creating a framebuffer...");
    let mut framebuffer = vec![0; WIDTH * HEIGHT];

    let mut t: f64 = 0.0;
    println!("Opening a window...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_framebuffer(&mut framebuffer, WIDTH, HEIGHT, t);
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
        if t >= 1.0 {
            t = 0.0;
        } else {
            t += 0.05;
        }
        thread::sleep(Duration::from_millis(20));
    }
}
