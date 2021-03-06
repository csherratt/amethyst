//! Launches a new renderer window.

extern crate amethyst_renderer as renderer;
extern crate cgmath;
extern crate genmesh;
extern crate winit;

use cgmath::{Deg, Matrix4, Vector3};
use cgmath::prelude::InnerSpace;
use genmesh::{MapToVertices, Triangulate, Vertices};
use genmesh::generators::SphereUV;
use renderer::prelude::*;
use renderer::vertex::PosNormTangTex;

fn main() {
    use std::time::{Duration, Instant};
    use winit::{Event, EventsLoop, WindowEvent};

    let mut events = EventsLoop::new();
    let mut renderer = Renderer::new(&events).expect("Renderer create");
    let pipe = renderer
        .create_pipe(
            Pipeline::build().with_stage(
                Stage::with_backbuffer()
                    .clear_target([0.0, 0.0, 0.0, 1.0], 2.0)
                    .with_model_pass(pass::DrawShaded::<PosNormTangTex>::new()),
            ),
        )
        .expect("Pipeline create");

    let verts = gen_sphere(64, 64);
    let mesh = renderer.create_mesh(Mesh::build(&verts)).expect(
        "Mesh create",
    );

    let mut scene = Scene::default();
    let alb = Texture::from_color_val([1.0; 4]);

    for i in 0..5 {
        for j in 0..5 {
            let roughness = 1.0f32 * (i as f32 / 4.0f32);
            let metallic = 1.0f32 * (j as f32 / 4.0f32);
            let pos = Matrix4::from_translation(
                [2.0f32 * (i - 2) as f32, 2.0f32 * (j - 2) as f32, 0.0].into(),
            ) * Matrix4::from_scale(0.8);

            let rog = Texture::from_color_val([roughness; 4]);
            let met = Texture::from_color_val([metallic; 4]);
            let mtl = renderer
                .create_material(
                    MaterialBuilder::new()
                        .with_albedo(alb.clone())
                        .with_roughness(rog)
                        .with_metallic(met),
                )
                .expect("Material create");
            let model = Model {
                mesh: mesh.clone(),
                material: mtl,
                pos: pos,
            };
            scene.add_model(model);
        }
    }

    let light = PointLight {
        center: [6.0, 6.0, -6.0].into(),
        intensity: 6.0,
        color: [0.8, 0.0, 0.0].into(),
        ..PointLight::default()
    };
    scene.add_light(light);

    let light = PointLight {
        center: [6.0, -6.0, -6.0].into(),
        intensity: 5.0,
        color: [0.0, 0.3, 0.7].into(),
        ..PointLight::default()
    };
    scene.add_light(light);

    scene.add_camera(Camera {
        eye: [0.0, 0.0, -12.0].into(),
        proj: Projection::perspective(1.3, Deg(60.0)).into(),
        forward: [0.0, 0.0, 1.0].into(),
        right: [1.0, 0.0, 0.0].into(),
        up: [0.0, 1.0, 0.0].into(),
    });

    let mut delta = Duration::from_secs(0);
    let mut running = true;
    while running {
        let start = Instant::now();

        events.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { .. } |
                    WindowEvent::Closed => running = false,
                    _ => (),
                }
            }
            _ => (),
        });

        renderer.draw(&scene, &pipe, delta);
        delta = Instant::now() - start;
    }
}

fn gen_sphere(u: usize, v: usize) -> Vec<PosNormTangTex> {
    SphereUV::new(u, v)
        .vertex(|(x, y, z)| {
            let normal = Vector3::from([x, y, z]).normalize();
            let up = Vector3::from([0.0, 1.0, 0.0]);
            let tangent = normal.cross(up).cross(normal);
            PosNormTangTex {
                a_position: [x, y, z],
                a_normal: normal.into(),
                a_tangent: tangent.into(),
                a_tex_coord: [0.1, 0.1],
            }
        })
        .triangulate()
        .vertices()
        .collect()
}
