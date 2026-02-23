use crate::{
  level::{
    GravityWell, LensingMaterial, LensingSettings, MAX_GRAVITY_WELLS,
    PostProcessScreen, Velocity, actors::Player,
  },
  prelude::*,
};

use bevy::{
  camera::{
    ImageRenderTarget, RenderTarget, ScalingMode, visibility::RenderLayers,
  },
  core_pipeline::tonemapping::Tonemapping,
  post_process::bloom::{Bloom, BloomCompositeMode},
  render::{
    render_resource::{
      Extent3d, TextureDescriptor, TextureDimension, TextureFormat,
      TextureUsages,
    },
    storage::ShaderStorageBuffer,
    view::{ColorGrading, ColorGradingGlobal, Hdr},
  },
};

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum CameraSystems {
  Follow,
}

pub fn plugin(app: &mut App) {
  app
    .add_systems(Startup, setup)
    .add_systems(PostUpdate, follow.in_set(CameraSystems::Follow))
    .add_systems(Update, resize_render_target);
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PostProcessCamera;

fn setup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<LensingMaterial>>,
  mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
  windows: Single<&Window>,
) {
  let window = windows.into_inner();
  let size = Extent3d {
    width: window.physical_width().max(1),
    height: window.physical_height().max(1),
    ..default()
  };

  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: Some("Main Render Target"),
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Rgba16Float,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };
  image.resize(size);

  let mut projection = OrthographicProjection::default_2d();
  projection.scaling_mode =
    ScalingMode::FixedVertical { viewport_height: 50.0 };

  let image = images.add(image);

  commands.spawn((
    PrimaryCamera,
    Name::new("Game Camera"),
    Hdr,
    Camera {
      order: 0,
      clear_color: ClearColorConfig::Custom(Color::srgb(0.01, 0.01, 0.02)),
      ..default()
    },
    RenderTarget::Image(ImageRenderTarget {
      handle: image.clone(),
      scale_factor: 1.0,
    }),
    Projection::Orthographic(projection),
    RenderLayers::layer(0),
  ));

  commands.spawn((
    PostProcessCamera,
    Name::new("Post Process Camera"),
    Hdr,
    Camera { order: 1, clear_color: ClearColorConfig::Default, ..default() },
    Tonemapping::TonyMcMapface,
    Bloom {
      intensity: 0.25,
      low_frequency_boost: 0.6,
      low_frequency_boost_curvature: 0.4,
      high_pass_frequency: 0.8,
      composite_mode: BloomCompositeMode::EnergyConserving,
      ..default()
    },
    ColorGrading {
      global: ColorGradingGlobal {
        exposure: 0.15,
        post_saturation: 1.2,
        ..default()
      },
      ..default()
    },
    RenderLayers::layer(1),
  ));

  let material = materials.add(LensingMaterial {
    settings: LensingSettings::default(),
    wells: buffers.add(ShaderStorageBuffer::from(
      [GravityWell::default(); MAX_GRAVITY_WELLS],
    )),
    screen_texture: image,
  });

  commands.spawn((
    Name::new("Post Process Screen"),
    PostProcessScreen,
    Mesh2d(meshes.add(Rectangle::new(window.width(), window.height()))),
    MeshMaterial2d(material),
    Transform2D::from_xy(0.0, 0.0),
    RenderLayers::layer(1),
  ));
}

fn resize_render_target(
  window: Single<&Window>,
  lensing: Single<&MeshMaterial2d<LensingMaterial>>,
  mut screen: Single<&mut Mesh2d, With<PostProcessScreen>>,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  materials: ResMut<Assets<LensingMaterial>>,
) {
  let window = window.into_inner();
  let width = (window.physical_width() as u32).max(1);
  let height = (window.physical_height() as u32).max(1);

  if let Some(material) = materials.get(lensing.into_inner())
    && let Some(image) = images.get_mut(&material.screen_texture)
  {
    if image.texture_descriptor.size.width != width
      || image.texture_descriptor.size.height != height
    {
      let size = Extent3d { width, height, ..default() };
      image.resize(size);

      let logical_width = window.width().max(1.0);
      let logical_height = window.height().max(1.0);
      **screen =
        Mesh2d(meshes.add(Rectangle::new(logical_width, logical_height)));
    }
  }
}

pub fn follow(
  time: Res<Time>,
  player: Single<(&Transform2D, &Velocity), With<Player>>,
  camera: Single<&mut Transform2D, (With<PrimaryCamera>, Without<Player>)>,
) {
  let (player, velocity) = player.into_inner();
  let mut camera = camera.into_inner();

  let lookahead = 0.3;
  let target_pos = player.translation + velocity.0 * lookahead;

  let lerp_speed = 5.0;
  camera.translation = camera
    .translation
    .lerp(target_pos, 1.0 - (-lerp_speed * time.delta_secs()).exp());
}
