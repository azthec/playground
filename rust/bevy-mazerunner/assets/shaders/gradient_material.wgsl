#import bevy_render::view::View;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,  // clip-space position
    @location(0) local_pos: vec2<f32>,       // mesh-local position
};

@vertex
fn vertex(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4(position, 1.0);
    out.local_pos = position.xy;
    return out;
}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> origin: vec2<f32>; // custom center
@group(2) @binding(2) var<uniform> max_distance: f32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = distance(in.local_pos, origin);
    // let alpha = clamp(1.0 - dist / max_distance, 0.0, 1.0);
    let alpha = 1.0 - smoothstep(50., max_distance, dist);
    return vec4(material_color.rgb, material_color.a * alpha);
}

