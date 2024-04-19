use minifb::{Key, Window, WindowOptions};
use std::iter;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const SEGMENT_LENGTH: usize = 5;
const INTERVAL_MILLIS: u64 = 1;

pub struct State {
    turn_index: usize,
    position: (isize, isize),  // pixel coordinates
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
    width: isize,
    height: isize,
    segment_length: usize,
    state: &mut State,
    turns: &Vec<Turn>,
) -> () {
    if state.turn_index >= turns.len() {
        return;
    }
    // update framebuffer
    let color = (1.0, 1.0, 1.0);
    if state.position.0 >= 0
        && state.position.1 >= 0
        && state.position.0 < width
        && state.position.1 < height
    {
        framebuffer[(state.position.0 + state.position.1 * width) as usize] = to_bgra(color);
    }
    // update state
    state.segment_progress += 1;
    state.position.0 += state.direction.0;
    state.position.1 += state.direction.1;
    if state.segment_progress >= segment_length {
        state.direction = turn(state.direction, turns[state.turn_index]);
        state.turn_index += 1;
        state.segment_progress = 0;
    }
}

fn flip(turn: Turn) -> Turn {
    match turn {
        Turn::L => Turn::R,
        Turn::R => Turn::L,
    }
}

fn next_turn_sequence(turns: &Vec<Turn>) -> Vec<Turn> {
    let turn_flipped = turns.into_iter().rev().map(|&t| flip(t));
    return turns
        .clone()
        .into_iter()
        .chain(iter::once(Turn::R))
        .chain(turn_flipped)
        .collect();
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
        position: (
            ((WIDTH / 2) as isize).try_into().unwrap(),
            ((HEIGHT / 2) as isize).try_into().unwrap(),
        ),
        direction: (1, 0),
        segment_progress: 0,
    };

    println!("Initializing turn sequence...");
    let mut turns = vec![Turn::R]; // base case

    println!("Opening a window...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if state.turn_index >= turns.len() {
            turns = next_turn_sequence(&turns);
        }
        update(
            &mut framebuffer,
            WIDTH.try_into().unwrap(),
            HEIGHT.try_into().unwrap(),
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
