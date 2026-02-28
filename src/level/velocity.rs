use crate::{level::CameraSystems, prelude::*};

#[derive(
  SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
pub enum VelocitySystems {
  Modify,
  Apply,
}

pub fn plugin(app: &mut App) {
  app.configure_sets(
    PostUpdate,
    (VelocitySystems::Modify, VelocitySystems::Apply).chain(),
  );

  app.add_systems(
    PostUpdate,
    apply_velocity.in_set(VelocitySystems::Apply).before(CameraSystems::Follow),
  );
}

#[derive(Component, Reflect, Default, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(pub Vec2);

pub fn apply_velocity(
  mut query: Query<(&mut Transform2D, &mut Velocity)>,
  time: Res<Time>,
) {
  query.par_iter_mut().for_each(|(mut transform, mut velocity)| {
    transform.translation += velocity.0 * time.delta_secs();
    velocity.0 = Vec2::ZERO;
  });
}
