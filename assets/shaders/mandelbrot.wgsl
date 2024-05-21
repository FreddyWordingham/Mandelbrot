#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/settings.wgsl"::COLOUR_MULTIPLIER

@group(2) @binding(0) var<uniform> quad_colour: vec4<f32>;
@group(2) @binding(1) var<uniform> world_scale: f32;
@group(2) @binding(2) var<uniform> real_coord: f32;
@group(2) @binding(3) var<uniform> imag_coord: f32;
@group(2) @binding(4) var<uniform> max_iterations: i32;
@group(2) @binding(5) var quad_texture: texture_2d<f32>;
@group(2) @binding(6) var base_colour_sampler: sampler;

fn mandelbrot(c: vec2<f32>) -> f32 {
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var iterations: i32 = 0;

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
    let m = mandelbrot(in.world_position.xy * world_scale - vec2<f32>(1.0 + real_coord, 1.0 + imag_coord) );
    return quad_colour * textureSample(quad_texture, base_colour_sampler, in.uv) * COLOUR_MULTIPLIER * vec4<f32>(1.0, 1.0, 1.0, m);
}
