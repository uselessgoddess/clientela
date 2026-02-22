use {
  crate::prelude::*,
  bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
  },
};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
    .register_type::<BackgroundMaterial>()
    .add_systems(Update, update_background_offset.in_set(Systems::Update));
}

#[derive(Reflect, Asset, AsBindGroup, Debug, Clone)]
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
  assets: &Res<LevelAssets>,
  materials: &mut ResMut<Assets<BackgroundMaterial>>,
  meshes: &mut ResMut<Assets<Mesh>>,
) {
  let bg_size = 1000.0;

  let material = materials.add(BackgroundMaterial {
    color: LinearRgba::new(0.2, 0.3, 0.4, 1.0),
    offset: Vec2::ZERO,
    grid_size: 5.0,
    line_thickness: 0.1,
    scale: bg_size, // default width is 50.0
  });

  commands.spawn((
    Name::new("Procedural Background"),
    Background,
    Mesh2d(meshes.add(Rectangle::new(bg_size, bg_size))),
    MeshMaterial2d(material),
    Transform2D::from_xy(0.0, 0.0).with_layer(-100.0),
  ));
}

fn update_background_offset(
  camera: Single<&Transform2D, With<PrimaryCamera>>,
  mut background_query: Query<
    (&mut Transform2D, &MeshMaterial2d<BackgroundMaterial>),
    (With<Background>, Without<PrimaryCamera>),
  >,
  mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
  let cam = camera.into_inner();

  for (mut bg, handle) in background_query.iter_mut() {
    bg.translation = cam.translation;

    if let Some(mat) = materials.get_mut(handle) {
      mat.offset = Vec2::new(cam.translation.x, -cam.translation.y);
    }
  }
}
