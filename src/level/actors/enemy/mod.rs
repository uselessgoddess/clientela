mod assets;
mod constellation;
mod state;

use std::f32::consts::TAU;

use crate::{
  level::{
    Collider, CollisionLayers, Level, SpatialGrid, Velocity,
    actors::{EnemyKind, Player, SpawnEnemy},
    physics::{self, GridEntry, PhysicsSystems},
  },
  prelude::*,
};

use constellation::Constellation;

pub fn plugin(app: &mut App) {
  app.add_plugins((assets::plugin,));
  app
    .add_systems(Update, spawn.in_set(Systems::Spawn))
    .add_systems(Update, movement.in_set(Systems::Update))
    .add_systems(Update, separation.in_set(PhysicsSystems::Update))
    .add_observer(on_spawn);
}

#[derive(Component, Reflect)]
#[require(Stats)]
pub struct Enemy;

#[derive(Component, Reflect)]
#[require(Velocity)]
pub struct Stats {
  pub speed: f32,
  pub attack: Attack,
}

impl Default for Stats {
  fn default() -> Self {
    // todo!> use config loading
    Self { speed: 2.0, attack: default() }
  }
}

// todo!> serializable parameter
#[derive(Reflect)]
pub struct Attack {
  pub range: f32,
  pub damage: f32,
}

impl Default for Attack {
  fn default() -> Self {
    // todo!> use solid user/enemy radius or else use ray/shape casting
    Self { range: (0.5 + 0.33 + 0.25), damage: 1.0 }
  }
}

fn spawn(query: Query<(Entity, &Enemy), Added<Enemy>>, mut commands: Commands) {
  for (entity, _enemy) in query.iter() {
    commands.entity(entity).insert((
      YSort::default(),
      Velocity::default(),
      CollisionLayers::new(
        physics::Layer::ENEMY,
        physics::Layer::PROJECTILE | physics::Layer::PLAYER,
      ),
    ));
  }
}

fn movement(
  player: Single<&Transform2D, With<Player>>,
  mut enemies: Query<
    (&mut Transform2D, &Stats),
    (With<Enemy>, Without<Player>),
  >,
  time: Res<Time>,
) {
  let player = player.into_inner();
  for (mut enemy, stats) in enemies.iter_mut() {
    let diff = player.translation - enemy.translation;
    enemy.translation +=
      diff.normalize_or_zero() * stats.speed * time.delta_secs();
  }
}

fn separation(
  mut enemies: Query<(&mut Transform2D, &Collider), With<Enemy>>,
  grid: Res<SpatialGrid>,
  time: Res<Time>,
) {
  let push_strength = 15.0; // TODO: configurable in `Stats`

  for (mut transform, &Collider(radius)) in enemies.iter_mut() {
    let mut push_vector = Vec2::ZERO;
    let pos = transform.translation;

    for entry in grid.query_nearest(pos) {
      if pos == entry.pos {
        continue;
      }

      let dist_sq = pos.distance_squared(entry.pos);
      let radius = radius + entry.radius;

      if dist_sq < radius * radius && dist_sq > f32::EPSILON {
        let dist = dist_sq.sqrt();
        push_vector += (pos - entry.pos) / dist * (radius - dist);
      }
    }

    transform.translation += push_vector * push_strength * time.delta_secs();
  }
}

fn on_spawn(
  spawn: On<SpawnEnemy>,
  assets: Res<assets::EnemyAssets>,
  mut commands: CommandsOf<Level>,
) {
  let (kind, pos) = (spawn.enemy, spawn.pos);

  match kind {
    EnemyKind::Swarm => {
      let (mesh, material) = assets.swarm.clone();
      let radius = 0.3;
      commands.spawn((Name::new("Swarm"), Enemy)).insert((
        Stats { speed: 3.5, attack: default() },
        Transform2D::from_translation(pos).with_scale(Vec2::splat(radius)),
        Collider(radius),
        Mesh2d(mesh),
        MeshMaterial2d(material),
      ));
    }
    EnemyKind::Constellation(nodes) => {
      let (mesh, material) = assets.swarm.clone();

      let nodes: Vec<_> = (0..nodes)
        .map(|i| {
          let offset = Vec2::from_angle((TAU / nodes as f32) * i as f32) * 2.0;
          let radius = 0.3;

          commands
            .spawn((Name::new("Constellation Node"), Enemy))
            .insert((
              Stats { speed: 1.5, attack: default() },
              Transform2D::from_translation(pos + offset)
                .with_scale(Vec2::splat(radius)),
              Collider(radius),
              Mesh2d(mesh.clone()),
              MeshMaterial2d(material.clone()),
            ))
            .id()
        })
        .collect();

      commands.spawn(Constellation(nodes));
    }
    _ => {}
  }
}
