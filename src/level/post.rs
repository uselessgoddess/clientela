use crate::{level::actors::ForceField, prelude::*};

use bevy::{
  prelude::*,
  render::{
    render_resource::{AsBindGroup, ShaderType},
    storage::ShaderStorageBuffer,
  },
  shader::ShaderRef,
  sprite_render::{Material2d, Material2dPlugin},
};

pub const MAX_GRAVITY_WELLS: usize = 256;

pub fn plugin(app: &mut App) {
  app
    .add_plugins(Material2dPlugin::<LensingMaterial>::default())
    .add_systems(Update, update_lensing.in_set(Systems::Update));
}

#[derive(ShaderType, Default, Debug, Copy, Clone)]
pub struct GravityWell {
  pub position: Vec2,
  pub strength: f32,
  pub radius: f32,
}

#[derive(ShaderType, Debug, Clone)]
pub struct LensingSettings {
  pub camera_pos: Vec2,
  pub viewport_size: Vec2,
  pub time: f32,
  pub well_count: u32,
  pub _pad: Vec2,
  pub wells: [GravityWell; MAX_GRAVITY_WELLS],
}

impl Default for LensingSettings {
  fn default() -> Self {
    Self {
      camera_pos: Vec2::ZERO,
      viewport_size: Vec2::ZERO,
      time: 0.0,
      well_count: 0,
      _pad: Vec2::ZERO,
      wells: [GravityWell::default(); _],
    }
  }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct LensingMaterial {
  #[uniform(0)]
  pub settings: LensingSettings,
  #[texture(1)]
  #[sampler(2)]
  pub screen_texture: Handle<Image>,
}

impl Material2d for LensingMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/lensing.wgsl".into()
  }
}

#[derive(Component)]
pub struct PostProcessScreen;

fn update_lensing(
  fields: Query<(&Transform2D, &ForceField)>,
  camera: Single<(&Projection, &Transform2D), With<PrimaryCamera>>,
  screen: Single<&MeshMaterial2d<LensingMaterial>, With<PostProcessScreen>>,
  mut materials: ResMut<Assets<LensingMaterial>>,
  time: Res<Time>,
) {
  let (
    Projection::Orthographic(proj),
    &Transform2D { translation: camera_pos, .. },
  ) = camera.into_inner()
  else {
    return warn!("failed to update lensing: projection must be orthographic");
  };
  let viewport_size = proj.area.size();

  let culling = (viewport_size.length() * 1.5).powi(2);
  let mut wells = [GravityWell::default(); MAX_GRAVITY_WELLS];
  let mut count = 0;

  for (tf, force) in fields.iter() {
    let dist = tf.translation.distance_squared(camera_pos);
    if dist < culling && count < MAX_GRAVITY_WELLS {
      wells[count] = GravityWell {
        position: tf.translation,
        strength: force.strength * 1.0,
        radius: force.radius / 20.0,
      };
      count += 1;
    }
  }

  if let Some(material) = materials.get_mut(screen.into_inner()) {
    material.settings = LensingSettings {
      time: time.elapsed_secs(),
      camera_pos,
      viewport_size,
      well_count: count as u32,
      wells,
      ..default()
    };
  }
}
