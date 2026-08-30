//! A shader and a custom material that uses it to make a pretty animated effect.

use bevy::{
    app::{App, Startup},
    asset::{Asset, Assets},
    camera::Camera3d,
    color::LinearRgba,
    ecs::system::{Commands, ResMut},
    math::{primitives::Cuboid, Vec3},
    mesh::Mesh,
    mesh::Mesh3d,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    pbr::{Material, MaterialPlugin, MeshMaterial3d},
    transform::components::Transform,
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
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/kaleidoscope3d.wesl".into()
    }
}

// Spawn an entity using `CustomMaterial`.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        Transform::from_xyz(0.0, 0.5, 0.0),
        MeshMaterial3d(materials.add(CustomMaterial {
            color: LinearRgba::RED,
        })),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}
