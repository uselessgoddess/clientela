pub mod enemy;
pub mod env;
mod palette;
pub mod player;

use crate::prelude::*;

pub use {enemy::Enemy, env::Brick, player::Player};

pub fn plugin(app: &mut App) {
  app.add_plugins((player::plugin, enemy::plugin, env::plugin));
}
