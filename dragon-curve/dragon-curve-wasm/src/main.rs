#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
// We stole most of this from:
// https://github.com/parasyte/pixels/tree/main/examples/minimal-web

const WIDTH: u32 = 2400;
const HEIGHT: u32 = 1600;
const SEGMENT_LENGTH: usize = 10;
const BATCH_SIZE: usize = 100;

pub struct Config {
    width: isize,
    height: isize,
    segment_length: usize,
}

pub struct State {
    position: (isize, isize),  // pixel coordinates
    direction: (isize, isize), // position + direction = next position
    starting_direction: (isize, isize),
    starting_position: (isize, isize),
    segment_progress: usize, // number of pixels into a segment
    t: usize,                // number of pixels into the curve
    turn_counter: i64,
    turn_state: i64,
    gradient: Vec<GradientStop>,
    countdown: usize, // number of iterations to wait before starting
    duration: usize,  // number of pixels to draw before resetting
}

impl State {
    fn new(
        starting_position: (isize, isize),
        starting_direction: (isize, isize),
        gradient: Vec<GradientStop>,
        countdown: usize,
        duration: usize,
    ) -> State {
        return State {
            starting_position: starting_position,
            starting_direction: starting_direction,
            position: starting_position,
            direction: starting_direction,
            segment_progress: 0,
            t: 0,
            turn_counter: 0,
            turn_state: 0,
            gradient: gradient,
            countdown: countdown,
            duration: duration,
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

fn to_rgba(color: (f64, f64, f64)) -> [u8; 4] {
    let r: u8 = (color.0 * 255.99) as u8;
    let g: u8 = (color.1 * 255.99) as u8;
    let b: u8 = (color.2 * 255.99) as u8;
    let a: u8 = 0xff;
    [r, g, b, a]
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

fn update(config: &Config, frame: &mut [u8], state: &mut State) -> () {
    if state.countdown > 0 {
        state.countdown -= 1;
        return;
    }

    if state.duration > 0 && state.t > state.duration {
        state.position = state.starting_position;
        state.starting_direction = turn(turn(state.starting_direction, Turn::R), Turn::R);
        state.direction = state.starting_direction;
        state.segment_progress = 0;
        state.turn_counter = 0;
        state.turn_state = 0;
        state.t = 0;
    }

    let mut d = (state.t as f64 + 1.0).log2();
    d -= d.floor();
    let color = get_gradient_color(&state.gradient, d);

    if state.position.0 >= 0
        && state.position.1 >= 0
        && state.position.0 < config.width
        && state.position.1 < config.height
    {
        let rgba = to_rgba(color);
        let i = ((state.position.0 + state.position.1 * config.width) * 4) as usize;
        frame[i..(i + 4)].copy_from_slice(&rgba);
    }
    // update state
    state.t += 1;
    state.segment_progress += 1;

    state.position.0 += state.direction.0;
    state.position.1 += state.direction.1;
    if state.segment_progress >= config.segment_length {
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
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");

        wasm_bindgen_futures::spawn_local(run());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();

        pollster::block_on(run());
    }
}

async fn run() {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("dragon curve, now on the web!")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("WindowBuilder error")
    };

    let window = Rc::new(window);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowExtWebSys;

        // Retrieve current width and height dimensions of browser client window
        let get_window_size = || {
            let client_window = web_sys::window().unwrap();
            LogicalSize::new(
                client_window.inner_width().unwrap().as_f64().unwrap(),
                client_window.inner_height().unwrap().as_f64().unwrap(),
            )
        };

        let window = Rc::clone(&window);

        // Initialize winit window with current dimensions of browser client
        window.set_inner_size(get_window_size());

        let client_window = web_sys::window().unwrap();

        // Attach winit canvas to body element
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        // Listen for resize event on browser client. Adjust winit window dimensions
        // on event trigger
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
            let size = get_window_size();
            window.set_inner_size(size)
        }) as Box<dyn FnMut(_)>);
        client_window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    let mut input = WinitInputHelper::new();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.as_ref());
        Pixels::new_async(WIDTH, HEIGHT, surface_texture)
            .await
            .expect("Pixels error")
    };

    let config = Config {
        width: WIDTH.try_into().unwrap(),
        height: HEIGHT.try_into().unwrap(),
        segment_length: SEGMENT_LENGTH,
    };

    let screen_center: (isize, isize) = (
        ((config.width / 2) as isize).try_into().unwrap(),
        ((config.height / 2) as isize).try_into().unwrap(),
    );

    // initialize states
    let mut states: Vec<State> = vec![
        State::new(
            screen_center,
            (1, 0),
            two_color_gradient(
                (255.0 / 255.0, 0.0, 0.0),           // red
                (255.0 / 255.0, 136.0 / 255.0, 0.0), // orange
            ),
            0,
            50000,
        ),
        State::new(
            screen_center,
            (0, 1),
            two_color_gradient(
                (80.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0), // blurple
                (187.0 / 255.0, 0.0, 80.0 / 255.0),         // pinkish
            ),
            10000,
            25000,
        ),
        State::new(
            screen_center,
            (-1, 0),
            two_color_gradient(
                (
                    153.0 / 255.0 / 5.0,
                    204.0 / 255.0 / 5.0,
                    255.0 / 255.0 / 5.0,
                ),
                (0.0 / 255.0, 176.0 / 255.0, 240.0 / 255.0), // #00b0f0
            ),
            0,
            50000,
        ),
        State::new(
            screen_center,
            (0, -1),
            two_color_gradient((0.1, 0.1, 0.1), (0.6, 0.6, 0.6)),
            10000,
            25000,
        ),
    ];

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state
            for _ in 0..BATCH_SIZE {
                for state in states.iter_mut() {
                    update(&config, pixels.frame_mut(), state);
                }
            }
            // and request a redraw
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
