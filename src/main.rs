use bevy::{
    prelude::*,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

use mandelbrot::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                // move_canvas,
                move_world_with_input,
                change_number_of_iterations,
                change_world_scale,
            ),
        )
        .run();
}

// Spawn a camera and a quad for displaying the shader.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Rendering quad
    let texture = Some(asset_server.load("textures/blank.png"));
    commands.spawn((
        Canvas,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(Vec3::splat(512.0)),
            material: materials.add(CustomMaterial::new(texture)),
            ..default()
        },
    ));
}
