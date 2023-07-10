#import bevy_sprite::mesh2d_view_bindings view
#import bevy_sprite::mesh2d_view_bindings globals
#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

fn palette(t: f32) -> vec3<f32> {
    var a = vec3(0.5, 0.5, 0.5);
    var b = vec3(0.5, 0.5, 0.5);
    var c = vec3(1.0, 1.0, 1.0);
    var d = vec3(0.263, 0.416, 0.557);

    return a + b * cos(6.28318 * (c * t + d)); 
}

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var uv = (mesh.position.xy * 2.0 - view.viewport.zw) / view.viewport.w; 
    var uv0 = uv;
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
