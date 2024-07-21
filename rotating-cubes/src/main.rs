use three_d::*;

const SIDE_COUNT: usize = 3;

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

    let mut stickers = Vec::new();

    let mut thin_cube = CpuMesh::cube();
    thin_cube
        .transform(&Mat4::from_nonuniform_scale(0.05, 0.05, 0.05))
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
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &stickers, &[&light0, &light1, &light2]);

        FrameOutput::default()
    });
}
