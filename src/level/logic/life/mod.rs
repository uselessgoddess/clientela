use crate::prelude::*;

mod health;
mod period;

pub use {health::Health, period::Lifetime};

pub fn plugin(app: &mut App) {
  app
    .add_plugins(health::plugin)
    .add_plugins(period::plugin)
    .add_systems(Update, death)
    .add_observer(damage);
}

#[derive(EntityEvent, Debug, Copy, Clone)]
pub struct Death {
  pub entity: Entity,
}

#[derive(EntityEvent, Debug, Copy, Clone)]
pub struct Damage {
  pub entity: Entity,
  pub value: f32,
}

fn damage(event: On<Damage>, mut query: Query<&mut Bar<Health>>) {
  if let Ok(mut health) = query.get_mut(event.entity) {
    health.dec(damage);
  }
}

fn death(query: Query<(Entity, &Bar<Health>)>, mut commands: Commands) {
  for (entity, health) in query.iter() {
    if health.is_empty() {
      commands.entity(entity).trigger(Death);
    }
  }
}
