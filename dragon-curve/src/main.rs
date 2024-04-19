use minifb::{Key, Window, WindowOptions};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const SEGMENT_LENGTH: usize = 30;
const INTERVAL_MILLIS: u64 = 10;

pub struct State {
    turn_index: usize,
    position: (usize, usize),  // pixel coordinates
    direction: (isize, isize), // position + direction = next position
    segment_progress: usize,   // number of pixels into a segment
}

#[derive(Clone, Copy)]
pub enum Turn {
    L,
    R,
}

fn turn(direction: (isize, isize), turn: Turn) -> (isize, isize) {
    match turn {
        Turn::L => (direction.1, -direction.0),
        Turn::R => (-direction.1, direction.0),
    }
}

fn to_bgra(color: (f64, f64, f64)) -> u32 {
    let r: u32 = (color.0 * 255.99) as u32;
    let g: u32 = (color.1 * 255.99) as u32;
    let b: u32 = (color.2 * 255.99) as u32;
    255 << 24 | r << 16 | g << 8 | b
}

fn update(
    framebuffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    segment_length: usize,
    state: &mut State,
    turns: &Vec<Turn>,
) -> () {
    if state.turn_index >= turns.len() {
        return;
    }
    // update framebuffer
    let color = (1.0, 1.0, 1.0);
    framebuffer[state.position.0 + state.position.1 * width] = to_bgra(color);
    // update state
    state.segment_progress += 1;
    state.position.0 = (state.position.0 as isize + state.direction.0) as usize;
    state.position.1 = (state.position.1 as isize + state.direction.1) as usize;
    if state.segment_progress >= segment_length {
        state.direction = turn(state.direction, turns[state.turn_index]);
        state.turn_index += 1;
        state.segment_progress = 0;
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

    println!("Initializing state...");
    let mut state = State {
        turn_index: 0,
        position: (WIDTH / 2, HEIGHT / 2),
        direction: (1, 0),
        segment_progress: 0,
    };

    println!("Initializing turn sequence...");
    let turns = vec![
        Turn::R,
        Turn::R,
        Turn::L,
        Turn::R,
        Turn::R,
        Turn::L,
        Turn::L,
        Turn::R,
        Turn::R,
        Turn::R,
        Turn::L,
        Turn::L,
        Turn::R,
        Turn::L,
        Turn::L,
    ];

    println!("Opening a window...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(
            &mut framebuffer,
            WIDTH,
            HEIGHT,
            SEGMENT_LENGTH,
            &mut state,
            &turns,
        );
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
        thread::sleep(Duration::from_millis(INTERVAL_MILLIS));
    }
}
