use {
  super::{Enemy, Stats},
  crate::{level::actors::Player, prelude::*},
};

pub fn plugin(app: &mut App) {
  app.add_systems(FixedUpdate, movement.in_set(Systems::Update));
}

#[derive(Component, Reflect)]
pub struct Boid(pub f32);

fn movement(
  mut query: Query<(&mut Transform2D, &Stats)>,
  player: Single<&Transform2D, With<Player>>,
  time: Res<Time>,
) {
  let player = player.into_inner();

  for (mut enemy, stats) in query.iter_mut() {
    enemy.translation += (player.translation - enemy.translation)
      * stats.speed
      * time.delta_secs();
  }
}
