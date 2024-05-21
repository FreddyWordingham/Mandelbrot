use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
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
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(512.0)),
        material: materials.add(CustomMaterial {
            quad_colour: Color::WHITE,
            quad_texture: Some(asset_server.load("textures/blank.png")),
        }),
        ..default()
    });
}

// This data is passed to the shader.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    quad_colour: Color,
    #[texture(1)]
    #[sampler(2)]
    quad_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}
