use three_d::*;

const SIDE_COUNT: usize = 3;
const CUBE_SPACING: f32 = 2.2;
const STICKER_WIDTH: f32 = 1.0;
const STICKER_THICKNESS: f32 = 0.05;

type Sticker = three_d::Gm<three_d::Mesh, three_d::PhysicalMaterial>;

const DIRECTIONS: [(isize, isize, isize); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];
const COLORS: [Srgba; 6] = [
    Srgba::WHITE,
    Srgba::new(255, 255, 0, 255),
    Srgba::GREEN,
    Srgba::BLUE,
    Srgba::RED,
    Srgba::new(255, 128, 0, 255),
];

struct Face {
    mesh: usize,
    direction: (isize, isize, isize),
}

struct Cubie {
    faces: Vec<Face>,
    position: (usize, usize, usize),
}

fn dir_to_range(d: isize) -> std::ops::Range<usize> {
    match d {
        0 => 0..SIDE_COUNT,
        -1 => 0..1,
        1 => (SIDE_COUNT - 1)..SIDE_COUNT,
        _ => panic!("Ahhh!!!!!!!"),
    }
}

fn dir_to_thickness(d: isize) -> f32 {
    match d {
        0 => STICKER_WIDTH,
        -1 => STICKER_THICKNESS,
        1 => STICKER_THICKNESS,
        _ => panic!("Ahhh!!!!!!!"),
    }
}

fn main() {
    let window = Window::new(WindowSettings {
        title: "rotating squares".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(25.00, 25.0, 25.0), // camera position
        vec3(0.0, 0.0, 0.0),     // camera target
        vec3(0.0, 1.0, 0.0),     // camera up
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let light0 = three_d::renderer::light::AmbientLight::new(&context, 0.8, Srgba::WHITE);
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));
    let light2 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.5, 0.5, 0.0));

    let mut cubies: Vec<Cubie> = vec![];
    let mut compartments: Vec<Vec<Vec<usize>>> = vec![];
    let mut stickers: Vec<Sticker> = vec![];
    for x in 0..SIDE_COUNT {
        let mut layer: Vec<Vec<usize>> = vec![];
        for y in 0..SIDE_COUNT {
            let mut row: Vec<usize> = vec![];
            for z in 0..SIDE_COUNT {
                let cubie = Cubie {
                    faces: vec![],
                    position: (x, y, z),
                };
                cubies.push(cubie);
                row.push(cubies.len() - 1);
            }
            layer.push(row);
        }
        compartments.push(layer);
    }

    let mut sticker_cube = CpuMesh::cube();
    sticker_cube
        .transform(&Mat4::from_nonuniform_scale(
            STICKER_WIDTH,
            STICKER_WIDTH,
            STICKER_WIDTH,
        ))
        .unwrap();

    for (direction, color) in DIRECTIONS.iter().zip(COLORS.iter()) {
        let (dir_x, dir_y, dir_z) = *direction;
        for x in dir_to_range(dir_x) {
            for y in dir_to_range(dir_y) {
                for z in dir_to_range(dir_z) {
                    let cubie = &mut cubies[compartments[x][y][z]];

                    let mut sticker = Gm::new(
                        Mesh::new(&context, &sticker_cube),
                        PhysicalMaterial::new(
                            &context,
                            &CpuMaterial {
                                albedo: *color,
                                ..Default::default()
                            },
                        ),
                    );

                    let default_axis: (isize, isize, isize) = (-1, 0, 0);
                    let (dax, day, daz) = default_axis;
                    let az = dir_x * day - dir_y * dax;
                    let ax = dir_x * day - dir_y * dax;
                    let az = dir_x * day - dir_y * dax;

                    sticker.set_transformation(
                        Mat4::from_translation(vec3(
                            CUBE_SPACING * (x as f32) + (dir_x as f32) * CUBE_SPACING / 2.0,
                            CUBE_SPACING * (y as f32) + (dir_y as f32) * CUBE_SPACING / 2.0,
                            CUBE_SPACING * (z as f32) + (dir_z as f32) * CUBE_SPACING / 2.0,
                        )) * Mat4::from_nonuniform_scale(
                            dir_to_thickness(dir_x) / STICKER_WIDTH,
                            dir_to_thickness(dir_y) / STICKER_WIDTH,
                            dir_to_thickness(dir_z) / STICKER_WIDTH,
                        ),
                    );

                    stickers.push(sticker);

                    cubie.faces.push(Face {
                        direction: *direction,
                        mesh: stickers.len() - 1,
                    })
                }
            }
        }
    }

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let time = (frame_input.accumulated_time * 0.001) as f32;
        // stickers.iter_mut().for_each(|m| m.animate(time));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, &stickers, &[&light0, &light1, &light2]);

        FrameOutput::default()
    });
}

fn old_main() {
    let window = Window::new(WindowSettings {
        title: "rotating squares".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(25.00, 25.0, 25.0), // camera position
        vec3(0.0, 0.0, 0.0),     // camera target
        vec3(0.0, 1.0, 0.0),     // camera up
        degrees(45.0),
        0.1,
        1000.0,
    );

    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let light0 = three_d::renderer::light::AmbientLight::new(&context, 0.8, Srgba::WHITE);
    let light1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.0, 0.5, 0.5));
    let light2 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, &vec3(0.5, 0.5, 0.0));

    let mut stickers = Vec::new();

    let mut thin_cube = CpuMesh::cube();
    thin_cube
        .transform(&Mat4::from_nonuniform_scale(
            STICKER_WIDTH,
            STICKER_WIDTH,
            STICKER_THICKNESS,
        ))
        .unwrap();

    for i in 0..(SIDE_COUNT * SIDE_COUNT) {
        let mut sticker = Gm::new(
            Mesh::new(&context, &thin_cube),
            PhysicalMaterial::new(
                &context,
                &CpuMaterial {
                    albedo: Srgba::WHITE,
                    ..Default::default()
                },
            ),
        );
        let x = (i % SIDE_COUNT) as f32;
        let y = ((i as f32 / SIDE_COUNT as f32).floor() as usize % SIDE_COUNT) as f32;
        let z = 3.0; // (i as f32 / SIDE_COUNT.pow(2) as f32).floor();
        println!("{:?} {:?} {:?}", x, y, z);

        sticker.set_animation(move |time| {
            Mat4::from_angle_x(Rad(time)) * Mat4::from_translation(vec3(x, y, z))
        });
        stickers.push(sticker);
    }

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        let time = (frame_input.accumulated_time * 0.001) as f32;
        stickers.iter_mut().for_each(|m| m.animate(time));

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, &stickers, &[&light0, &light1, &light2]);

        FrameOutput::default()
    });
}
