use crate::{level::actors::Player, prelude::*};

pub fn plugin(app: &mut App) {
  register(app);
}

fn register(app: &mut App) -> &mut App {
  app.register_type::<Palette>()
}

/// Brushes manager
#[derive(Component, Reflect)]
struct Palette;

fn spawn(
  query: Query<(Entity, &Player), Added<Palette>>,
  mut commands: Commands,
) {
  
}
