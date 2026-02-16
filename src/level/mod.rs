pub mod actors;
mod camera;
mod follow;

use crate::prelude::*;

pub use follow::{Follow, FollowMouse};

pub fn plugin(app: &mut App) {
  app.register_type::<LevelAssets>();
  app.configure_loading_state(
    LoadingStateConfig::new(Game::Loading)
      // -- assets --
      .load_collection::<LevelAssets>(),
  );

  app.add_plugins((camera::plugin, follow::plugin));
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

pub fn spawn_level(mut commands: Commands, assets: Res<LevelAssets>) {
  commands
    .spawn((Name::new("Level"), DespawnOnExit(Game::Gameplay), Level {}))
    .with_children(|parent| {
      //parent
      //  .spawn((Name::new("Player"), Player))
      //  .insert(Transform2D::from_xy(10.0, 0.0));
    });
}
