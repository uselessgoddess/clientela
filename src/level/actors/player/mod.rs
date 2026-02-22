mod assets;
mod input;
mod state;

use crate::prelude::*;

background_timer!(StepsTimer);

pub fn plugin(app: &mut App) {
  register(app)
    .add_plugins((assets::plugin, input::plugin, state::plugin))
    .add_systems(Update, (spawn).in_set(Systems::Spawn));
}

fn register(app: &mut App) -> &mut App {
  app
    .register_type::<Stats>()
    .register_type::<Player>()
    .register_timer::<StepsTimer>()
}

#[derive(Component, Reflect, Default, Clone)]
#[require(Stats)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct Stats {
  pub speed: f32,
}

impl Default for Stats {
  fn default() -> Self {
    Self { speed: 8.0 }
  }
}

fn spawn(
  query: Query<(Entity, &Player), Added<Player>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut commands: Commands,
) {
  let radius = 0.5;

  for (player, _) in query.iter() {
    let mesh = meshes.add(Circle::new(radius));
    let material = materials.add(Color::srgb(0.1, 0.2, 0.1));

    commands
      .entity(player)
      .insert((input::map(), state::Controller))
      .insert((Mesh2d(mesh), MeshMaterial2d(material)));
  }
}
