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
  pub force: f32,
  pub radius: f32,
}

impl ForceField {
  pub fn new(force: f32, radius: f32) -> Self {
    Self { force, radius }
  }

  pub fn horizon(&self) -> f32 {
    self.radius / 20.0
  }
}

fn apply_force_fields(
  fields: Query<(&Transform2D, &ForceField)>,
  mut actors: Query<(&Transform2D, &mut Velocity), Without<ForceField>>,
) {
  let fields_data: Vec<_> = fields
    .iter()
    .map(|(t, f)| {
      let horizon = f.horizon();
      (t.translation, f.radius * f.radius, f.force, horizon * horizon)
    })
    .collect();

  if fields_data.is_empty() {
    return;
  }

  actors.par_iter_mut().for_each(|(actor, mut velocity)| {
    let mut total_force = Vec2::ZERO;

    for &(field_pos, radius_sq, force, softening_sq) in &fields_data {
      let diff = field_pos - actor.translation;
      let dist_sq = diff.length_squared();

      if dist_sq < radius_sq {
        let dist = dist_sq.sqrt();
        let dir = if dist > f32::EPSILON { diff / dist } else { Vec2::ZERO };
        total_force += dir * force / (dist_sq + softening_sq)
          * (1.0 - dist_sq / radius_sq).max(0.0);
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
