struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    grid_opacity: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

const K1: f32 = 0.333333333;
const K2: f32 = 0.166666667;
const FBM_OCTAVES: i32 = 4;
// Normalize the four-octave amplitude sum toward 2.0.
const FBM_ENERGY: f32 = 1.05;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );

    var out: VertexOut;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = positions[vertex_index] * 0.5 + vec2<f32>(0.5);

    return out;
}

fn hash(input: vec3<f32>) -> vec3<f32> {
    var p3 = fract(input * vec3<f32>(0.1031, 0.11369, 0.13787));
    p3 += dot(p3, p3.yxx + 19.19);

    return -1.0 + 2.0 * fract(vec3<f32>(
        (p3.x + p3.y) * p3.z,
        (p3.x + p3.z) * p3.y,
        (p3.y + p3.z) * p3.x,
    ));
}

fn noise(n: vec2<f32>, time: f32) -> f32 {
    let p = vec3<f32>(n.x, n.y, time);
    let i = floor(p + (p.x + p.y + p.z) * K1);
    let d0 = p - (i - (i.x + i.y + i.z) * K2);
    let e = step(vec3<f32>(0.0), d0 - d0.yzx);
    let i1 = e * (1.0 - e.zxy);
    let i2 = 1.0 - e.zxy * (1.0 - e);
    let d1 = d0 - (i1 - K2);
    let d2 = d0 - (i2 - 2.0 * K2);
    let d3 = d0 - (1.0 - 3.0 * K2);
    let h = max(
        0.6 - vec4<f32>(
            dot(d0, d0),
            dot(d1, d1),
            dot(d2, d2),
            dot(d3, d3),
        ),
        vec4<f32>(0.0),
    );
    let q = h * h * h * h * vec4<f32>(
        dot(d0, hash(i)),
        dot(d1, hash(i + i1)),
        dot(d2, hash(i + i2)),
        dot(d3, hash(i + 1.0)),
    );

    return dot(vec4<f32>(50.0), q);
}

fn fbm(input: vec2<f32>, time: f32, scale: f32) -> f32 {
    var p = input * scale;
    var value = 0.0;
    var amplitude = 1.0;

    for (var octave = 0; octave < FBM_OCTAVES; octave += 1) {
        value += amplitude * abs(noise(p, time));
        p *= 2.0;
        amplitude *= 0.5;
    }

    return 1.0 - value * FBM_ENERGY;
}

fn technical_grid(position: vec2<f32>) -> f32 {
    let coordinate = vec2<i32>(i32(position.x + 1.0), i32(position.y));
    let bits = (coordinate.x ^ coordinate.y) & 168;

    return fwidth(f32(bits)) / 99.0;
}

@fragment
fn fragment_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let fragment_position = uv * uniforms.resolution;
    let reference_uv = fragment_position / max(uniforms.resolution.y, 1.0);
    let field = fbm(reference_uv * 12.0, uniforms.time * 0.5, -0.5);
    let grid = technical_grid(fragment_position) * clamp(uniforms.grid_opacity, 0.0, 1.0);
    let intensity = clamp(field + grid, 0.0, 1.0);

    return vec4<f32>(vec3<f32>(intensity), 1.0);
}
