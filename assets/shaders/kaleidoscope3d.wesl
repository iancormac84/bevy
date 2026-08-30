import bevy_pbr::render::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
};

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> material: vec4<f32>;

fn palette(t: f32) -> vec3<f32> {
    var a = vec3(0.5, 0.5, 0.5);
    var b = vec3(0.5, 0.5, 0.5);
    var c = vec3(1.0, 1.0, 1.0);
    var d = vec3(0.263, 0.416, 0.557);

    return a + b * cos(6.28318 * (c * t + d)); 
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // Force each face to use its own 0–1 UV range
    var uv = fract(mesh.uv);
    // Reintroduce a center (-1 to +1)
    var centered_uv = uv * 2.0 - 1.0;
    var uv0 = centered_uv * 1.5;
    var final_color = vec3(0.0);
    for (var i = 0; i < 4; i++) {
        uv = fract(uv * 1.5) - 0.5;

        var d = length(uv) * exp(-length(uv0));

        var col = palette(length(uv0) + f32(i) * 0.4 + globals.time * 0.4);
        
        d = sin(d * 8.0 + globals.time) / 8.0;
        d = abs(d);
        d = pow(0.01 / d, 1.2);

        final_color += col * d;
    }
    return vec4(final_color, 1.0);
}
