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
  let fields_data: Vec<_> = fields
    .iter()
    .map(|(t, f)| (t.translation, f.radius, f.strength, f.radius * f.radius))
    .collect();

  if fields_data.is_empty() {
    return;
  }

  actors.par_iter_mut().for_each(|(actor, mut velocity)| {
    let mut total_force = Vec2::ZERO;

    for &(field_pos, radius, strength, radius_sq) in &fields_data {
      let diff = actor.translation - field_pos;
      let dist_sq = diff.length_squared();

      if dist_sq < radius_sq {
        let dist = dist_sq.sqrt().max(0.5);
        let dir = diff / dist;

        let physical_force = strength / dist_sq;
        let edge_smoothing = (1.0 - dist / radius).max(0.0);

        total_force += -dir * physical_force * edge_smoothing;
      }
    }

    if total_force != Vec2::ZERO {
      velocity.0 += total_force;
    }
  });
}

fn debug_force_fields(
  fields: Query<(&Transform2D, &ForceField)>,
  mut gizmos: Gizmos,
) {
  for (&field, force) in fields.iter() {
    gizmos.circle_2d(field, force.radius, Color::WHITE);
  }
}
