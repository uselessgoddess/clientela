#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GravityWell {
    position: vec2<f32>,
    strength: f32,
    radius: f32,
}

struct BackgroundMaterial {
    color: vec4<f32>,
    offset: vec2<f32>,
    grid_size: f32,
    line_thickness: f32,
    scale: f32,
    time: f32,    
    well_count: i32,
}

@group(2) @binding(0) var<uniform> material: BackgroundMaterial;
@group(2) @binding(1) var<uniform> wells: array<GravityWell, 16>;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Координаты (как мы исправили в прошлый раз)
    let uv_centered = mesh.uv - 0.5;
    let uv_corrected = vec2(uv_centered.x, -uv_centered.y);
    var world_pos = (uv_corrected * material.scale) + material.offset;

    // Сохраняем оригинальную позицию для виньетки и прочего
    let original_pos = world_pos;

    var glow = vec3(0.0);
    var total_intensity = 0.0;


    // 2. Анимация гравитации
    for (var i = 0; i < material.well_count; i++) {
        let well = wells[i];
        
        let delta = world_pos - well.position; 
        let dist = length(delta);
        let dir = normalize(delta); // Направление ОТ центра воронки

        // --- ФИШКА 1: "Дыхание" радиуса ---
        // Радиус слегка пульсирует (синус времени). 
        // 3.0 - скорость пульсации, 0.1 - амплитуда (10% от радиуса).
        let pulse = sin(material.time * 1.0) * 0.10; 
        let effective_radius = well.radius * (1.0 + pulse);

        // Влияние: 1.0 в центре, 0.0 на краю
        let influence = smoothstep(effective_radius, 0.0, dist); 
        
        glow += vec3(0.2, 0.0, 0.5) * influence;


        if (influence > 0.0) {
            // --- ФИШКА 2: Защита от "Узла" (Сингулярности) ---
            // Мы не даем смещению стать больше, чем 80% расстояния до центра.
            // Это предотвращает выворачивание сетки наизнанку.
            // clamp(val, min, max)
            
            // Статичное искажение (Линза)
            let pinch_strength = min(-well.strength * influence, dist);
            world_pos -= dir * pinch_strength;

            // --- ФИШКА 3: Поток линий внутрь (Flow) ---
            // Мы смещаем координаты "вверх по течению". 
            // Это создает иллюзию, что сетка едет в дыру.
            // flow_speed зависит от influence (в центре быстрее).
            
            let flow_speed = 5.0; // Скорость засасывания линий
            let flow_offset = dir * flow_speed * influence * sign(-well.strength);
            
            // Добавляем поток к координатам
            total_intensity += influence;
            world_pos += flow_offset;
        }
    }

    let cell_coord = world_pos - round(world_pos / material.grid_size) * material.grid_size;
    let dist_to_edge = abs(cell_coord);
    let dist_to_line = (material.grid_size / 2.0) - max(dist_to_edge.x, dist_to_edge.y);
    let line_width_in_pixels = fwidth(dist_to_line / 20.0);
    let line_alpha = smoothstep(
        material.line_thickness - line_width_in_pixels, 
        material.line_thickness + line_width_in_pixels, 
        dist_to_line
    );

    // ... Виньетка по original_pos или uv ...
    let dist_from_center = distance(mesh.uv, vec2(0.5));
    let vignette = 1.0 - smoothstep(0.45, 0.5, dist_from_center);

    // Добавляем цвет в центре воронки (опционально - "Горизонт событий")
    // Если хочешь подсветить дыры
    
    
    let final_rgb = material.color.rgb * line_alpha + (glow * total_intensity * 0.3);
    let final_alpha = (line_alpha * 0.4) + (total_intensity * 0.1);

    return vec4<f32>(final_rgb * vignette, final_alpha * vignette);
}