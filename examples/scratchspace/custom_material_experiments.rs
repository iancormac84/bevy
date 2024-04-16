//! A shader and a custom material that uses it to make a pretty animated effect.

use bevy::{
    app::{App, Startup},
    asset::{Asset, Assets},
    color::LinearRgba,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::With,
        system::{Commands, Query, ResMut},
    },
    math::{primitives::Rectangle, Vec3},
    reflect::TypePath,
    render::{
        mesh::Mesh,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    // Uniform bindings must implement `ShaderType`, which will be used to convert the value to
    // its shader-compatible equivalent. Most core math types already implement `ShaderType`.
    #[uniform(0)]
    color: LinearRgba,
}

// All functions on `Material2d` have default impls. You only need to implement the
// functions that are relevant for your material.
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_experiments.wgsl".into()
    }
}

// Spawn an entity using `CustomMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    //SpriteBundle
    commands.spawn(Camera2dBundle::default());
    if let Ok(window) = primary_window.get_single() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
            transform: Transform::default().with_scale(Vec3::new(
                window.width(),
                window.height(),
                1024.,
            )),
            material: materials.add(CustomMaterial { color: LinearRgba::RED }),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}
