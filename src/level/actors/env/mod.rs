pub mod background;
mod brick;
mod force;

use crate::prelude::*;

pub use {
  background::{Background, BackgroundMaterial, spawn_background},
  brick::Brick,
};

pub fn plugin(app: &mut App) {
  app.add_plugins((brick::plugin, background::plugin, force::plugin));
}
