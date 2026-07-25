struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

const INV_TAU: f32 = 0.159154943;
const HALF_PI: f32 = 1.570796327;
const MARCH_STEPS: u32 = 32u;
const WARP_OCTAVES: u32 = 3u;
const MIN_DISTANCE: f32 = 0.01;
const MAX_TRAVEL: f32 = 12.0;
const MAX_RADIANCE: f32 = 1800.0;
const EXPOSURE: f32 = 0.0021875;

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

fn fast_cos(value: vec3<f32>) -> vec3<f32> {
    let phase = fract(value * INV_TAU + vec3<f32>(0.5)) * 2.0 - vec3<f32>(1.0);
    let squared = phase * phase;
    let polynomial = vec3<f32>(-4.888888889)
        + squared * (vec3<f32>(3.777777778) - squared * 0.888888889);

    return vec3<f32>(1.0) + squared * polynomial;
}

fn fast_sin(value: vec3<f32>) -> vec3<f32> {
    return fast_cos(value - vec3<f32>(HALF_PI));
}

@fragment
fn fragment_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let aspect = uniforms.resolution.x / max(uniforms.resolution.y, 1.0);
    let ray = normalize(vec3<f32>(
        (2.0 * uv.x - 1.0) * aspect,
        1.0 - 2.0 * uv.y,
        -1.0,
    ));
    var travel = 0.0;
    var radiance = vec3<f32>(0.0);

    for (var step = 0u; step < MARCH_STEPS; step += 1u) {
        var point = travel * ray;
        let bend = fast_sin(point.zzz + vec3<f32>(0.0, 2.0, 7.0));
        let original_xy = point.xy;
        point.x = original_xy.x * bend.x + original_xy.y * bend.y;
        point.y = original_xy.x * bend.z + original_xy.y * bend.x;

        var frequency = 7.5;
        for (var octave = 0u; octave < WARP_OCTAVES; octave += 1u) {
            point += fast_cos(
                ceil(point.yzx * frequency) + vec3<f32>(uniforms.time)
            ) / frequency;
            frequency *= 1.25;
        }

        let distance = max(length(point.xy + vec2<f32>(2.0)) / 9.0, MIN_DISTANCE);
        travel += distance;
        radiance += (
            fast_sin(point.zzz + vec3<f32>(0.0, 7.0, 8.0)) + vec3<f32>(1.0)
        ) / distance;

        if travel > MAX_TRAVEL
            || min(radiance.x, min(radiance.y, radiance.z)) > MAX_RADIANCE
        {
            break;
        }
    }

    return vec4<f32>(tanh(radiance * EXPOSURE), 1.0);
}
