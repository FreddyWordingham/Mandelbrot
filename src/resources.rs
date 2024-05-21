use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

// This data is passed to the shader.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub quad_colour: Color,
    #[uniform(1)]
    pub world_scale: f32,
    #[uniform(2)]
    pub real_coord: f32,
    #[uniform(3)]
    pub imag_coord: f32,
    #[uniform(4)]
    pub max_iterations: i32,
    #[texture(5)]
    #[sampler(6)]
    quad_texture: Option<Handle<Image>>,
}

impl CustomMaterial {
    pub fn new(texture: Option<Handle<Image>>) -> Self {
        Self {
            quad_colour: Color::WHITE,
            world_scale: 0.01,
            real_coord: 0.0,
            imag_coord: 0.0,
            max_iterations: 10,
            quad_texture: texture,
        }
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mandelbrot.wgsl".into()
    }
}
