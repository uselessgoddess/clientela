#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BackgroundMaterial {
    color: vec4<f32>,
    offset: vec2<f32>,
    grid_size: f32,
    line_thickness: f32,
    scale: f32,
    time: f32,
}

@group(2) @binding(0) var<uniform> material: BackgroundMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv_centered = mesh.uv - 0.5;
    let uv_corrected = vec2(uv_centered.x, -uv_centered.y);
    let world_pos = (uv_corrected * material.scale) + material.offset;

    // Рисуем бесконечную сетку
    let cell_coord = world_pos - round(world_pos / material.grid_size) * material.grid_size;
    let dist_to_edge = abs(cell_coord);
    let dist_to_line = (material.grid_size / 2.0) - max(dist_to_edge.x, dist_to_edge.y);
    let line_width_in_pixels = fwidth(dist_to_line / 20.0);
    
    let line_alpha = smoothstep(
        material.line_thickness - line_width_in_pixels, 
        material.line_thickness + line_width_in_pixels, 
        dist_to_line
    );

    // Цвет космической пустоты
    let bg_color = vec3(0.02, 0.02, 0.04);
    // Цвет неоновой сетки
    let grid_color = material.color.rgb;

    // Смешиваем
    let final_rgb = mix(bg_color, grid_color, line_alpha * 0.4);

    return vec4<f32>(final_rgb, 1.0);
}