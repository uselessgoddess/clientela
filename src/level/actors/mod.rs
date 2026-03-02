pub mod enemy;
pub mod env;
mod palette;
pub mod player;
mod spawner;

use crate::prelude::*;

pub use {
  enemy::Enemy,
  env::{Brick, ForceField},
  player::Player,
  spawner::{EnemyKind, SpawnEnemy},
};

pub fn plugin(app: &mut App) {
  app.add_plugins((
    player::plugin,
    spawner::plugin,
    enemy::plugin,
    env::plugin,
  ));
}
