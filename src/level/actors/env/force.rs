use crate::{
  level::{Velocity, VelocitySystems, actors::Enemy},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.add_systems(
    PostUpdate,
    apply_force_fields.in_set(VelocitySystems::Modify),
  );
  app.add_systems(Update, debug_force_fields.run_if(in_debug(D::L1)));
}

#[derive(Component)]
pub struct ForceField {
  pub radius: f32,
  pub strength: f32,
}

fn apply_force_fields(
  fields: Query<(&Transform2D, &ForceField)>,
  mut actors: Query<
    (&Transform2D, &mut Velocity),
    (With<Enemy>, Without<ForceField>),
  >,
) {
  for (field, force) in fields.iter() {
    for (actor, mut velocity) in actors.iter_mut() {
      let diff = actor.translation - field.translation;

      if diff.length() < force.radius {
        let intensity = (1.0 - diff.length() / force.radius).max(0.0);
        let dir = diff / diff.length();

        velocity.0 += dir * force.strength * intensity;
      }
    }
  }
}

fn debug_force_fields(
  fields: Query<(&Transform2D, &ForceField)>,
  mut gizmos: Gizmos,
) {
  for (&field, force) in fields.iter() {
    gizmos.circle_2d(field, force.radius, Color::WHITE);
  }
}
