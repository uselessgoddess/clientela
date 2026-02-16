pub mod audio;
pub mod cache;
mod debug;
mod dev;
pub mod ecs;
mod system;
mod timer;
pub mod ysort;

use crate::prelude::*;

pub use {
  audio::{Music, SoundEffect, music, sound_effect, sound_effect_with},
  debug::{D, in_debug},
  ecs::{PausableSystems, Pause, Transform2D, Transform2DSystems},
  timer::{LazyTimer, RegisterTimer},
  ysort::{BACKGROUND_OFFSET, YSort},
};

pub(crate) use timer::background_timer;

pub fn plugin(app: &mut App) {
  app.add_plugins((
    system::plugin,
    debug::plugin,
    ecs::plugin,
    audio::plugin,
    ysort::plugin,
  ));

  if debug::dev() {
    app.add_plugins(dev::plugin);
  }

  app.configure_sets(
    Update,
    (
      Systems::Spawn,
      Systems::Timers,
      Systems::Input,
      Systems::Update,
      Systems::Watch,
    )
      .chain()
      .in_set(PausableSystems),
  );
}

#[derive(Component)]
#[require(Camera2d)]
pub struct PrimaryCamera;

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum Systems {
  /// Spawn systems
  Spawn,
  /// Tick timers
  Timers,
  /// Record player input
  Input,
  /// Process game logic
  Update,
  /// Systems that prefer to be `On`, but not
  Watch,
}
