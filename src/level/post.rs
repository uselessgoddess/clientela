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

pub const MAX_GRAVITY_WELLS: usize = 4096;

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

#[derive(ShaderType, Default, Debug, Clone)]
pub struct LensingSettings {
  pub camera_pos: Vec2,
  pub viewport_size: Vec2,
  pub time: f32,
  pub well_count: u32,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct LensingMaterial {
  #[uniform(0)]
  pub settings: LensingSettings,
  #[storage(1, read_only)]
  pub wells: Handle<ShaderStorageBuffer>,
  #[texture(2)]
  #[sampler(3)]
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
  mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
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
  let mut wells: Vec<_> = fields
    .iter()
    .filter_map(|(tf, force)| {
      let dist = tf.translation.distance_squared(camera_pos);
      (dist < culling).then(|| GravityWell {
        position: tf.translation,
        strength: force.strength * 1.0,
        radius: force.radius / 20.0,
      })
    })
    .collect();

  // FIXME: sort nearest wells
  wells.truncate(MAX_GRAVITY_WELLS);
  wells.resize(MAX_GRAVITY_WELLS, GravityWell::default());

  if let Some(material) = materials.get_mut(screen.into_inner())
    && let Some(buffer) = buffers.get_mut(&material.wells)
  {
    material.settings = LensingSettings {
      time: time.elapsed_secs(),
      camera_pos,
      viewport_size,
      well_count: wells.len() as u32,
    };
    buffer.set_data(wells);
  }
}
