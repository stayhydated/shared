struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    grid_opacity: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

const MARCH_STEPS: i32 = 40;
const RAY_DEPTH: f32 = 2.0;
const LAYER_SPACING: f32 = 0.72;
const LAYER_WEIGHT: f32 = 1.72;
const WARP_SCALE: f32 = 1.36;
const PATH_SCALE: f32 = 19.0;
const MIN_CLOUD_ASPECT: f32 = 0.95;
const FBM_ROTATION: mat2x2<f32> = mat2x2<f32>(
    0.7451744, 0.66686964,
    -0.66686964, 0.7451744
);

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct LayerState {
    color: vec3<f32>,
};

struct LayerSample {
    density: f32,
    color: vec3<f32>,
};

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );

    var out: VertexOut;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = positions[vertex_index] * 0.5 + vec2<f32>(0.5);
    return out;
}

fn rotate2(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let basis = mat2x2<f32>(
        cos(angle), sin(angle),
        -sin(angle), cos(angle)
    );

    return basis * p;
}

fn fragment_position(uv: vec2<f32>) -> vec2<f32> {
    return uv * uniforms.resolution;
}

fn viewport_aspect() -> f32 {
    return uniforms.resolution.x / max(uniforms.resolution.y, 1.0);
}

fn portrait_focus() -> f32 {
    return 1.0 - smoothstep(0.72, 1.08, viewport_aspect());
}

fn centered_screen(position: vec2<f32>) -> vec2<f32> {
    let uv = position / max(uniforms.resolution, vec2<f32>(1.0));
    let centered = uv * 2.0 - vec2<f32>(1.0);
    let cloud_aspect = max(viewport_aspect(), MIN_CLOUD_ASPECT);

    return centered * vec2<f32>(cloud_aspect, 1.0);
}

fn camera_ray(uv: vec2<f32>) -> vec3<f32> {
    let position = fragment_position(uv);
    let centered = centered_screen(position);
    return normalize(vec3<f32>(centered, RAY_DEPTH));
}

fn hash12(p: vec2<f32>) -> f32 {
    let p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    let q = p3 + vec3<f32>(dot(p3, p3.yzx + vec3<f32>(33.33)));
    return fract((q.x + q.y) * q.z);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let cell = floor(p);
    let local = fract(p);
    let curve = local * local * (3.0 - 2.0 * local);

    let a = hash12(cell);
    let b = hash12(cell + vec2<f32>(1.0, 0.0));
    let c = hash12(cell + vec2<f32>(0.0, 1.0));
    let d = hash12(cell + vec2<f32>(1.0, 1.0));
    let x1 = mix(a, b, curve.x);
    let x2 = mix(c, d, curve.x);

    return mix(x1, x2, curve.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var sample_point = p;

    for (var octave = 0; octave < 3; octave = octave + 1) {
        value = value + amplitude * value_noise(sample_point);
        sample_point = FBM_ROTATION * (sample_point * 2.03 + vec2<f32>(4.7, -2.9));
        amplitude = amplitude * 0.5;
    }

    return value;
}

fn layer_depth(layer: i32) -> f32 {
    let layer_id = f32(layer);
    let offset = hash12(vec2<f32>(layer_id, 19.7)) * 0.24;
    return 0.15 + (layer_id + offset) * LAYER_SPACING;
}

fn layer_position(ray: vec3<f32>, depth: f32, slow_time: f32) -> vec2<f32> {
    var p = ray.xy * depth;
    p = rotate2(p, depth + slow_time);

    let warp = vec2<f32>(
        fbm(p.yx * WARP_SCALE + vec2<f32>(depth * 0.07, slow_time)),
        fbm(p * WARP_SCALE + vec2<f32>(-slow_time, depth * 0.05))
    ) - vec2<f32>(0.5);

    let cell = floor(p.yx * 0.34 + depth * 0.11);
    let folded = sin(cell + vec2<f32>(uniforms.time, uniforms.time + 1.7)) * 1.8;

    return p + warp * 5.0 + folded;
}

fn path_center(depth: f32, focus: f32) -> vec2<f32> {
    let t = depth * 0.21 + uniforms.time * 0.11;
    let drift = vec2<f32>(
        sin(t * 1.7) + 0.65 * sin(t * 0.41 + 2.3),
        cos(t * 1.3) + 0.55 * sin(t * 0.53 - 1.4)
    );
    let base = mix(vec2<f32>(-8.0), vec2<f32>(-5.2, -6.4), vec2<f32>(focus));
    let drift_scale = mix(2.2, 1.65, focus);

    return base + drift * drift_scale;
}

fn sample_layer(
    ray: vec3<f32>,
    depth: f32,
    focus: f32,
    slow_time: f32,
    path_scale: f32,
) -> LayerSample {
    let p = layer_position(ray, depth, slow_time);
    let path_distance = length(p - path_center(depth, focus)) / path_scale;
    let filament = 0.45 + 0.95 * fbm(p * 0.22 + vec2<f32>(depth * 0.04, -depth * 0.03));
    let shell = 1.0 - smoothstep(6.0, 52.0, depth);
    let density = filament * shell * LAYER_WEIGHT / max(path_distance, 0.001);
    let color = sin(depth + vec3<f32>(0.0, 6.0, 7.0)) + vec3<f32>(0.1);

    return LayerSample(density, color);
}

fn integrate_layers(ray: vec3<f32>) -> LayerState {
    var state = LayerState(vec3<f32>(0.0));
    let focus = portrait_focus();
    let slow_time = uniforms.time * 0.08;
    let path_scale = PATH_SCALE * mix(1.0, 1.18, focus);

    for (var layer = 0; layer < MARCH_STEPS; layer = layer + 1) {
        let depth = layer_depth(layer);
        let sample = sample_layer(ray, depth, focus, slow_time, path_scale);
        state.color = state.color + sample.color * sample.density;
    }

    return state;
}

fn soft_clip3(value: vec3<f32>) -> vec3<f32> {
    return value / (vec3<f32>(1.0) + abs(value));
}

fn vignette_system(uv: vec2<f32>) -> f32 {
    let falloff = length((uv - vec2<f32>(0.5)) * vec2<f32>(1.2, 1.0));
    return 1.0 - smoothstep(0.2, 1.55, falloff);
}

fn map_color(color: vec3<f32>, uv: vec2<f32>) -> vec3<f32> {
    let mapped = soft_clip3(color / 950.0);
    let vignette = vignette_system(uv);
    return mapped * (0.5 + 0.5 * vignette);
}

fn grid_overlay(uv: vec2<f32>) -> f32 {
    let fragment = fragment_position(uv);
    let coordinate = vec2<i32>(i32(fragment.x + 1.0), i32(fragment.y));
    let bits = (coordinate.x ^ coordinate.y) & 168;
    return fwidth(f32(bits)) / 99.0;
}

@fragment
fn fragment_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let ray = camera_ray(uv);
    let state = integrate_layers(ray);
    let final_color = map_color(state.color, uv);
    let grid = grid_overlay(uv) * clamp(uniforms.grid_opacity, 0.0, 1.0);
    let grid_tint = vec3<f32>(0.05, 0.18, 0.34) * grid;

    return vec4<f32>(clamp(final_color + grid_tint, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
