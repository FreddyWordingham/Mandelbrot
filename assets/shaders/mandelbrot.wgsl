#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(1) var quad_texture: texture_2d<f32>;
@group(2) @binding(2) var base_colour_sampler: sampler;

fn mandelbrot(c: vec2<f32>) -> f32 {
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var iterations: i32 = 0;
    let max_iterations: i32 = 100;

    while (length(z) < 2.0 && iterations < max_iterations) {
        let x = (z.x * z.x - z.y * z.y) + c.x;
        let y = (2.0 * z.x * z.y) + c.y;
        z = vec2<f32>(x, y);
        iterations += 1;
    }

    return f32(iterations) / f32(max_iterations);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let m = mandelbrot(in.uv * 2.0 - vec2<f32>(1.0, 1.0));
    return quad_colour * textureSample(quad_texture, base_colour_sampler, in.uv) * COLOUR_MULTIPLIER * vec4<f32>(1.0, 1.0, 1.0, m);
}
