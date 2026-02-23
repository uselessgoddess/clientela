use crate::{level::actors::ForceField, prelude::*};

use bevy::{
  prelude::*,
  render::render_resource::{AsBindGroup, ShaderType},
  shader::ShaderRef,
  sprite_render::{Material2d, Material2dPlugin},
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
    .register_type::<BackgroundMaterial>()
    .add_systems(Update, update_background.in_set(Systems::Update));
}

const MAX_GRAVITY_WELLS: usize = 16;

#[derive(ShaderType, Default, Debug, Clone, Reflect)]
pub struct GravityWell {
  pub position: Vec2,
  pub strength: f32,
  pub radius: f32,
}

#[derive(Reflect, Asset, AsBindGroup, Default, Debug, Clone)]
pub struct BackgroundMaterial {
  #[uniform(0)]
  pub color: LinearRgba,
  #[uniform(0)]
  pub offset: Vec2,
  #[uniform(0)]
  pub grid_size: f32,
  #[uniform(0)]
  pub line_thickness: f32,
  #[uniform(0)]
  pub scale: f32,
  #[uniform(0)]
  pub time: f32,
}

impl Material2d for BackgroundMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/background.wgsl".into()
  }
}

#[derive(Component)]
pub struct Background;

pub fn spawn_background(
  commands: &mut Commands,
  _assets: &Res<LevelAssets>,
  materials: &mut ResMut<Assets<BackgroundMaterial>>,
  meshes: &mut ResMut<Assets<Mesh>>,
) {
  let bg_size = 1000.0;

  let material = materials.add(BackgroundMaterial {
    color: LinearRgba::new(0.6, 1.2, 4.0, 1.0),
    offset: Vec2::ZERO,
    grid_size: 2.0,
    line_thickness: 0.05,
    scale: bg_size,
    ..default()
  });

  commands.spawn((
    Name::new("Procedural Background"),
    Background,
    Mesh2d(meshes.add(Rectangle::new(bg_size, bg_size))),
    MeshMaterial2d(material),
    Transform2D::from_xy(0.0, 0.0).with_layer(-100.0),
  ));
}

fn update_background(
  camera: Single<&Transform2D, (With<PrimaryCamera>, Without<Background>)>,
  mut background: Query<
    (&mut Transform2D, &MeshMaterial2d<BackgroundMaterial>),
    (With<Background>, Without<PrimaryCamera>),
  >,
  mut materials: ResMut<Assets<BackgroundMaterial>>,
  time: Res<Time>,
) {
  let cam = camera.into_inner().translation;

  for (mut bg, handle) in background.iter_mut() {
    bg.translation = cam;

    if let Some(mat) = materials.get_mut(handle) {
      mat.offset = cam;
      mat.time = time.elapsed_secs();
    }
  }
}
