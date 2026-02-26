use crate::{
  level::{Velocity, VelocitySystems},
  prelude::*,
};

pub fn plugin(app: &mut App) {
  app.add_systems(
    PostUpdate,
    apply_force_fields.in_set(VelocitySystems::Modify),
  );
  app.add_systems(Update, debug_force_fields.run_if(in_debug(D::L1)));
}

#[derive(Component, Reflect)]
#[require(Transform)]
pub struct ForceField {
  pub radius: f32,
  pub strength: f32,
}

impl ForceField {
  pub const MIN_EFFECTIVE_FORCE: f32 = 0.1;

  pub fn from_strength(strength: f32) -> Self {
    let radius = (strength.abs() / Self::MIN_EFFECTIVE_FORCE).sqrt();
    Self { radius, strength }
  }

  pub fn from_radius(radius: f32, is_white: bool) -> Self {
    let strength = Self::MIN_EFFECTIVE_FORCE
      * (radius * radius)
      * if is_white { -1.0 } else { 1.0 };
    Self { radius, strength }
  }
}

fn apply_force_fields(
  fields: Query<(&Transform2D, &ForceField)>,
  mut actors: Query<(&Transform2D, &mut Velocity), Without<ForceField>>,
) {
  for (field, force) in fields.iter() {
    for (actor, mut velocity) in actors.iter_mut() {
      let diff = actor.translation - field.translation;

      if diff.length() < force.radius {
        let dir = diff / diff.length();
        let dist = diff.length().max(0.5);
        let physical_force = force.strength / (dist * dist);
        let edge_smoothing = (1.0 - dist / force.radius).max(0.0);
        velocity.0 += -dir * physical_force * edge_smoothing;
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
