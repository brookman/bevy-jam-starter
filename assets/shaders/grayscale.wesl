//! Fullscreen grayscale shader
import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct Grayscale {
    // 0 = original colors, 1 = fully gray.
    strength: f32,
    @if(SIXTEEN_BYTE_ALIGNMENT)
    _webgl2_padding: vec3<f32>
}

@group(0) @binding(2) var<uniform> settings: Grayscale;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(screen_texture, texture_sampler, in.uv);
    let luminance = dot(color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    return vec4<f32>(mix(color.rgb, vec3<f32>(luminance), settings.strength), color.a);
}
