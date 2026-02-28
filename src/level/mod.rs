pub mod actors;
mod camera;
mod follow;
pub mod logic;
mod physics;
mod post;
mod velocity;

use crate::{level::actors::Player, prelude::*};

pub use {
  camera::CameraSystems,
  follow::{Follow, FollowMouse},
  physics::{Collider, CollisionEvent, CollisionLayers, SpatialGrid},
  post::{
    GravityWell, LensingMaterial, LensingSettings, MAX_GRAVITY_WELLS,
    PostProcessScreen,
  },
  velocity::{Velocity, VelocitySystems},
};

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      // -- assets --
      .load_collection::<LevelAssets>(),
  );

  app.add_plugins((
    camera::plugin,
    velocity::plugin,
    physics::plugin,
    follow::plugin,
    logic::plugin,
    post::plugin,
  ));
}

// todo!> find better name
#[derive(Component, Default, Copy, Clone)]
#[require(Obstacle)]
pub struct Difficulty;

#[derive(Component, Default, Copy, Clone)]
pub struct Obstacle;

#[derive(Component)]
#[require(Visibility, Transform)]
pub struct Level {}

pub use crate::actors::env::{BackgroundMaterial, spawn_background};

pub fn spawn_level(
  mut commands: Commands,
  assets: Res<LevelAssets>,
  // --
  mut meshes: ResMut<Assets<Mesh>>,
  mut bg_materials: ResMut<Assets<BackgroundMaterial>>,
) {
  commands
    .spawn((Name::new("Level"), DespawnOnExit(Game::Gameplay), Level {}))
    .with_children(|parent| {
      parent
        .spawn((Name::new("Player"), Player))
        .insert(Transform2D::from_xy(0.0, 0.0));
    });

  spawn_background(&mut commands, &assets, &mut bg_materials, &mut meshes);
}
