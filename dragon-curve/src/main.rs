use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn to_bgra(color: (f64, f64, f64)) -> u32 {
    let r: u32 = (color.0 * 255.99) as u32;
    let g: u32 = (color.1 * 255.99) as u32;
    let b: u32 = (color.2 * 255.99) as u32;
    255 << 24 | r << 16 | g << 8 | b
}

fn render(width: usize, height: usize) -> Vec<u32> {
    let mut framebuffer = vec![0; width * height];
    for y in 0..height {
        for x in 0..width {
            // just do a gradient
            let color = (y as f64 / height as f64, x as f64 / width as f64, 0 as f64);
            framebuffer[y * width + x] = to_bgra(color);
        }
    }
    framebuffer
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

    println!("Populating a framebuffer...");
    let framebuffer = render(WIDTH, HEIGHT);

    println!("Opening a window...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
