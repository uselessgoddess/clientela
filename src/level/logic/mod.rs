mod life;

use crate::prelude::*;

pub use life::{Damage, Death, Health, Lifetime};

pub fn plugin(app: &mut App) {
  app.add_plugins(life::plugin);
}
