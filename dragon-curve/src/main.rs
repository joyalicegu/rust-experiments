use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;
const SEGMENT_LENGTH: usize = 1;
const BATCH_SIZE: usize = 1000;

// colors
const WHITE: (f64, f64, f64) = (1.0, 1.0, 1.0);
const RED: (f64, f64, f64) = (1.0, 0.0, 0.0);
const YELLOW: (f64, f64, f64) = (1.0, 1.0, 0.0);
const GREEN: (f64, f64, f64) = (0.0, 1.0, 0.0);
const CYAN: (f64, f64, f64) = (0.0, 1.0, 1.0);
const BLUE: (f64, f64, f64) = (0.0, 0.0, 1.0);
const MAGENTA: (f64, f64, f64) = (1.0, 0.0, 1.0);

pub struct State {
    position: (isize, isize),  // pixel coordinates
    direction: (isize, isize), // position + direction = next position
    segment_progress: usize,   // number of pixels into a segment
    t: usize,                  // number of pixels into the curve
    turn_counter: i64,
    turn_state: i64,
}

impl State {
    fn new(direction: (isize, isize)) -> State {
        return State {
            position: (
                ((WIDTH / 2) as isize).try_into().unwrap(),
                ((HEIGHT / 2) as isize).try_into().unwrap(),
            ),
            direction: direction,
            segment_progress: 0,
            t: 0,
            turn_counter: 0,
            turn_state: 0,
        };
    }
}

#[derive(Clone, Copy)]
pub struct GradientStop {
    depth: f64,
    color: (f64, f64, f64),
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

fn lerp_f64(u: f64, v: f64, t: f64) -> f64 {
    v * t + u * (1.0 - t)
}

fn lerp_color(u: (f64, f64, f64), v: (f64, f64, f64), t: f64) -> (f64, f64, f64) {
    (
        lerp_f64(u.0, v.0, t),
        lerp_f64(u.1, v.1, t),
        lerp_f64(u.2, v.2, t),
    )
}

fn two_color_gradient(a: (f64, f64, f64), b: (f64, f64, f64)) -> Vec<GradientStop> {
    vec![
        GradientStop {
            depth: 0.0,
            color: a,
        },
        GradientStop {
            depth: 1.0 / 2.0,
            color: b,
        },
        GradientStop {
            depth: 1.0,
            color: a,
        },
    ]
}

fn get_gradient_color(gradient: &Vec<GradientStop>, depth: f64) -> (f64, f64, f64) {
    for i in 1..gradient.len() {
        if gradient[i].depth >= depth {
            let t = (depth - gradient[i - 1].depth) / (gradient[i].depth - gradient[i - 1].depth);
            return lerp_color(gradient[i - 1].color, gradient[i].color, t);
        }
    }
    panic!("Invalid gradient depth: {:?}", depth);
}

fn update(
    framebuffer: &mut Vec<u32>,
    width: isize,
    height: isize,
    segment_length: usize,
    state: &mut State,
    gradient: &Vec<GradientStop>,
) -> () {
    // update framebuffer
    let mut d = (state.t as f64 + 1.0).log2();
    d -= d.floor();
    let color = get_gradient_color(&gradient, d);
    if state.position.0 >= 0
        && state.position.1 >= 0
        && state.position.0 < width
        && state.position.1 < height
    {
        framebuffer[(state.position.0 + state.position.1 * width) as usize] = to_bgra(color);
    }
    // update state
    state.t += 1;
    state.segment_progress += 1;

    state.position.0 += state.direction.0;
    state.position.1 += state.direction.1;
    if state.segment_progress >= segment_length {
        // bits that differ when you increment the turn counter
        let bits = state.turn_counter ^ (state.turn_counter + 1);
        // most significant bit
        let bit = (bits + 1) >> 1;

        let current_turn = if (state.turn_state & bit) != 0 {
            Turn::L
        } else {
            Turn::R
        };

        state.turn_state ^= bit; // flip the bit
        state.turn_counter += 1;
        state.direction = turn(state.direction, current_turn);
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
    let mut state = State::new((1, 0));
    let mut state2 = State::new((0, 1));
    let mut state3 = State::new((-1, 0));
    let mut state4 = State::new((0, -1));

    let _solid_gradient = vec![
        GradientStop {
            depth: 0.0,
            color: WHITE,
        },
        GradientStop {
            depth: 1.0,
            color: WHITE,
        },
    ];

    let _hsv_gradient = vec![
        GradientStop {
            depth: 0.0,
            color: RED,
        },
        GradientStop {
            depth: 1.0 / 6.0,
            color: YELLOW,
        },
        GradientStop {
            depth: 2.0 / 6.0,
            color: GREEN,
        },
        GradientStop {
            depth: 3.0 / 6.0,
            color: CYAN,
        },
        GradientStop {
            depth: 4.0 / 6.0,
            color: BLUE,
        },
        GradientStop {
            depth: 5.0 / 6.0,
            color: MAGENTA,
        },
        GradientStop {
            depth: 1.0,
            color: RED,
        },
    ];

    println!("Opening a window...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..BATCH_SIZE {
            update(
                &mut framebuffer,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                SEGMENT_LENGTH,
                &mut state,
                &two_color_gradient(
                    RED,
                    (255.0 / 255.0, 136.0 / 255.0, 0.0), // orange
                ),
            );

            update(
                &mut framebuffer,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                SEGMENT_LENGTH,
                &mut state2,
                &two_color_gradient(
                    (80.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0), // blurple
                    (187.0 / 255.0, 0.0, 80.0 / 255.0),         // pinkish
                ),
            );

            update(
                &mut framebuffer,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                SEGMENT_LENGTH,
                &mut state3,
                &two_color_gradient(
                    (
                        153.0 / 255.0 / 5.0,
                        204.0 / 255.0 / 5.0,
                        255.0 / 255.0 / 5.0,
                    ),
                    (0.0 / 255.0, 176.0 / 255.0, 240.0 / 255.0), // 00b0f0
                ),
            );

            update(
                &mut framebuffer,
                WIDTH.try_into().unwrap(),
                HEIGHT.try_into().unwrap(),
                SEGMENT_LENGTH,
                &mut state4,
                &two_color_gradient((0.1, 0.1, 0.1), (0.6, 0.6, 0.6)),
            );
        }
        window
            .update_with_buffer(&framebuffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
