use {super::Stats, crate::prelude::*};

pub fn plugin(app: &mut App) {
  app.add_systems(FixedUpdate, movement.in_set(Systems::Update));
}

#[derive(Component, Reflect)]
pub struct Boid(pub f32);

fn movement(// mut controllers: Query<&mut Controller>,
  // query: Query<(Entity, &Transform2D, &Stats, &Boid)>,
) {
}
