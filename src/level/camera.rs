use crate::{
  level::{Velocity, actors::Player},
  prelude::*,
};

use bevy::camera::ScalingMode;

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum CameraSystems {
  Follow,
}

pub fn plugin(app: &mut App) {
  app
    .add_systems(Startup, setup)
    .add_systems(PostUpdate, follow.in_set(CameraSystems::Follow));
}

fn setup(mut commands: Commands) {
  let mut projection = OrthographicProjection::default_2d();
  projection.scaling_mode =
    ScalingMode::FixedVertical { viewport_height: 50.0 };

  commands.spawn((
    PrimaryCamera,
    Name::new("Camera"),
    Projection::Orthographic(projection),
  ));
}

pub fn follow(
  time: Res<Time>,
  player: Single<(&Transform2D, &Velocity), With<Player>>,
  camera: Single<&mut Transform2D, (With<PrimaryCamera>, Without<Player>)>,
) {
  let (player, velocity) = player.into_inner();
  let mut camera = camera.into_inner();

  let lookahead = 0.3;
  let target_pos = player.translation + velocity.0 * lookahead;

  let lerp_speed = 5.0;
  camera.translation = camera
    .translation
    .lerp(target_pos, 1.0 - (-lerp_speed * time.delta_secs()).exp());
}
