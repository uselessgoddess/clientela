// #import bevy_sprite::mesh2d_vertex_output::VertexOutput
// 
// struct GravityWell {
//     position: vec2<f32>, // Мировая позиция
//     strength: f32,           // Масса (сила искажения)
//     radius: f32,         // Радиус горизонта событий (черная зона)
// }
// 
// struct LensingSettings {
//     camera_pos: vec2<f32>,
//     viewport_size: vec2<f32>,
//     time: f32,
//     well_count: u32,
// }
// 
// @group(2) @binding(0) var<uniform> settings: LensingSettings;
// // storage позволяет передавать тысячи элементов!
// @group(2) @binding(1) var<storage, read> wells: array<GravityWell>;
// @group(2) @binding(2) var screen_texture: texture_2d<f32>;
// @group(2) @binding(3) var screen_sampler: sampler;
// 
// @fragment
// fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
//     let uv = mesh.uv;
//     
//     // Переводим UV в мировые координаты (с учетом позиции камеры)
//     let world_pos = (uv - 0.5) * vec2(settings.viewport_size.x, -settings.viewport_size.y) + settings.camera_pos;
// 
//     var total_uv_offset = vec2(0.0);
//     var event_horizon = 0.0;
//     var total_force = 0.0;
// 
//     // Считаем влияние всех 4096 дыр
//     for (var i = 0u; i < settings.well_count; i++) {
//         let well = wells[i];
//         
//         let delta = world_pos - well.position;
//         let dist = length(delta);
//         
//         // Физика: Искажение обратно пропорционально расстоянию.
//         // max() предотвращает деление на 0 в самом центре дыры.
//         let safe_dist = max(dist, well.radius);
//         
//         // Сила гравитационной линзы
//         let force = well.strength / (safe_dist * safe_dist);
//         let dir = delta / dist; // Направление ОТ дыры
//         
//         // Переводим мировое смещение в UV-смещение
//         let uv_shift = (dir * force) / settings.viewport_size;
//         total_uv_offset -= uv_shift; // Минус, так как линза затягивает свет
//         total_force += force;
// 
//         // Если мы за горизонтом событий - свет не выходит
//         if dist < well.radius {
//             event_horizon = 1.0;
//         }
//     }
// 
//     let final_uv = uv + total_uv_offset;
// 
//     // --- Хроматическая аберрация (разложение спектра около дыр) ---
//     // Чем сильнее сила притяжения (total_force), тем больше аберрация
//     let aberration_strength = clamp(total_force * 0.05, 0.0, 0.05);
//     
//     let color_r = textureSample(screen_texture, screen_sampler, final_uv - total_uv_offset * aberration_strength).r;
//     let color_g = textureSample(screen_texture, screen_sampler, final_uv).g;
//     let color_b = textureSample(screen_texture, screen_sampler, final_uv + total_uv_offset * aberration_strength).b;
//     
//     var color = vec4<f32>(color_r, color_g, color_b, 1.0);
// 
//     // Внутри горизонта событий абсолютная пустота
//     color = mix(color, vec4<f32>(0.0, 0.0, 0.0, 1.0), event_horizon);
// 
//     // Добавим легкое свечение "аккреционного диска" по краям линзы
//     let glow = clamp(total_force * 0.005, 0.0, 0.3);
//     color += vec4(0.2, 0.5, 1.0, 0.0) * glow; // Неоново-синее свечение
// 
//     return color;
// }

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GravityWell {
    position: vec2<f32>,
    strength: f32,
    radius: f32,
}

// Порядок строго как в Rust
struct LensingSettings {
    camera_pos: vec2<f32>,
    viewport_size: vec2<f32>,
    time: f32,
    well_count: u32,
}

@group(2) @binding(0) var<uniform> settings: LensingSettings;
@group(2) @binding(1) var<storage, read> wells: array<GravityWell>;
@group(2) @binding(2) var screen_texture: texture_2d<f32>;
@group(2) @binding(3) var screen_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    // Мировая позиция пикселя экрана
    let world_pos = settings.camera_pos + (uv - 0.5) * vec2(settings.viewport_size.x, -settings.viewport_size.y);

    var total_uv_offset = vec2<f32>(0.0);
    var is_in_event_horizon = 0.0;
    var accretion_glow = vec3<f32>(0.0);
    var max_distortion = 0.0;

    for (var i = 0u; i < settings.well_count; i++) {
        let well = wells[i];
        let delta = world_pos - well.position;
        let dist = length(delta);
        
        let rs = well.radius;             // Радиус Шварцшильда
        let photon_sphere = rs * 1.5;     // Фотонная сфера

        if (dist <= rs && well.strength > 0.0) {
            is_in_event_horizon = 1.0;
        } else if (well.strength > 0.0) {
            // Эмуляция гравитационной линзы
            let force = well.strength / (dist * dist + 0.01); 
            let dir = delta / dist;
            let uv_shift = (dir * force) / settings.viewport_size;
            total_uv_offset -= uv_shift;
            
            max_distortion = max(max_distortion, length(uv_shift));

            // Свечение аккреционного диска
            let glow_dist = abs(dist - photon_sphere);
            let pulse = 1.0 + sin(settings.time * 6.0 - dist * 2.0) * 0.2;
            let glow_intensity = exp(-glow_dist * 5.0) * (well.strength * 0.015) * pulse;
            
            accretion_glow += vec3(0.2, 0.6, 1.0) * glow_intensity; // Плазменный синий
        }
    }

    let final_uv = uv + total_uv_offset;

    // --- Хроматическая аберрация (разложение спектра около дыр) ---
    let ca_strength = min(max_distortion * 40.0, 0.04); 
    let uv_dir = normalize(total_uv_offset + vec2(0.0001));

    let color_r = textureSample(screen_texture, screen_sampler, final_uv - uv_dir * ca_strength).r;
    let color_g = textureSample(screen_texture, screen_sampler, final_uv).g;
    let color_b = textureSample(screen_texture, screen_sampler, final_uv + uv_dir * ca_strength).b;
    
    var color = vec3<f32>(color_r, color_g, color_b);

    // Добавляем свечение поверх искажения
    color += accretion_glow;

    // Горизонт событий перекрывает всё (абсолютная чернота)
    color = mix(color, vec3<f32>(0.0, 0.0, 0.0), is_in_event_horizon);

    // Легкая кинематографичная виньетка
    let vignette = 1.0 - smoothstep(0.4, 0.7, distance(uv, vec2(0.5)));
    color *= mix(0.8, 1.0, vignette);

    return vec4<f32>(color, 1.0);
}