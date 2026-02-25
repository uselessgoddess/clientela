#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GravityWell {
    position: vec2<f32>,
    strength: f32,
    radius: f32,
}

struct LensingSettings {
    camera_pos: vec2<f32>,
    viewport_size: vec2<f32>,
    time: f32,
    well_count: u32,
    _pad: vec2<f32>, 
    wells: array<GravityWell, 256>, 
}

@group(2) @binding(0) var<uniform> settings: LensingSettings;
@group(2) @binding(1) var screen_texture: texture_2d<f32>;
@group(2) @binding(2) var screen_sampler: sampler;

fn mirror_coord(v: f32) -> f32 {
    let t = fract(v * 0.5) * 2.0;
    return 1.0 - abs(t - 1.0);
}

fn mirror_uv(uv: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(mirror_coord(uv.x), mirror_coord(uv.y));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let world_pos = settings.camera_pos + (uv - 0.5) * vec2(settings.viewport_size.x, -settings.viewport_size.y);

    var total_uv_offset = vec2<f32>(0.0);
    var is_black_hole = 0.0;
    var is_white_hole = 0.0;
    var total_glow = vec3<f32>(0.0);
    var max_distortion = 0.0;

    for (var i = 0u; i < settings.well_count; i++) {
        let well = settings.wells[i];
        let delta = world_pos - well.position;
        let dist = length(delta);
        let rs = well.radius;
        
        let abs_strength = abs(well.strength);
        let force_dir = sign(well.strength); 

        if (dist <= rs && abs_strength > 0.0) {
            if (force_dir > 0.0) {
                is_black_hole = 1.0;
            } else {
                is_white_hole = 1.0; 
            }
        } else if (abs_strength > 0.0) {
            
            if (dist > 30.0) { continue; }

            let safe_dist = max(dist, rs * 1.02); 
            let force = abs_strength / (safe_dist * safe_dist);
            let dir = delta / dist;
            
            let raw_shift = (dir * force) / settings.viewport_size;
            
            let max_shift = 0.25; 
            let shift_len = length(raw_shift);
            let smooth_shift_len = max_shift * (1.0 - exp(-shift_len / max_shift));
            
            let uv_shift = dir * smooth_shift_len * force_dir;
            total_uv_offset -= uv_shift; 
            
            max_distortion = max(max_distortion, smooth_shift_len);

            let photon_sphere = rs * 1.5;
            let glow_dist = abs(dist - photon_sphere);
            let pulse = 1.0 + sin(settings.time * 6.0 - dist * 2.0 * force_dir) * 0.15;
            let glow = exp(-glow_dist * 3.5) * (abs_strength * 0.015) * pulse;
            
            // --- HDR ЦВЕТА (Значения > 1.0 вызывают Bloom) ---
            let color_bh = vec3(1.0, 3.0, 10.0); // Пылающая синяя плазма
            let color_wh = vec3(10.0, 8.0, 4.0); // Золотое солнце
            let current_glow_color = mix(color_wh, color_bh, step(0.0, well.strength));
            
            total_glow += current_glow_color * glow;
        }
    }

    let final_uv = uv + total_uv_offset;

    let ca_strength = min(max_distortion * 0.8, 0.015);
    let uv_dir = normalize(total_uv_offset + vec2(0.0001));

    let r_uv = mirror_uv(final_uv - uv_dir * ca_strength);
    let g_uv = mirror_uv(final_uv);
    let b_uv = mirror_uv(final_uv + uv_dir * ca_strength);

    let color_r = textureSample(screen_texture, screen_sampler, r_uv).r;
    let color_g = textureSample(screen_texture, screen_sampler, g_uv).g;
    let color_b = textureSample(screen_texture, screen_sampler, b_uv).b;
    
    var final_color = vec3<f32>(color_r, color_g, color_b);

    // Добавляем свечение поверх всего
    final_color += total_glow;

    // --- СИНГУЛЯРНОСТИ ---
    // Черная дыра поглощает свет
    final_color = mix(final_color, vec3<f32>(0.0, 0.0, 0.0), is_black_hole);
    // Белая дыра ослепляет (HDR: 30.0! Это вызовет массивное гало блума)
    final_color = mix(final_color, vec3<f32>(30.0, 28.0, 25.0), is_white_hole); 

    let vignette = 1.0 - smoothstep(0.4, 0.75, distance(uv, vec2(0.5)));
    final_color *= mix(0.7, 1.0, vignette);

    return vec4<f32>(final_color, 1.0);
}