use {
  crate::{level::actors::Player, prelude::*},
  rand::Rng,
  std::f32::consts::TAU,
};

pub fn plugin(app: &mut App) {
  app.insert_resource(WaveManager::default()); // TODO: make configurable
  app.add_systems(Update, wave_director.in_set(Systems::Spawn));
}

#[derive(Debug, Copy, Clone)]
pub enum EnemyKind {
  Swarm,
  Pulsar,
  Singularity,
  Constellation(u32),
}

#[derive(Clone, Debug)]
pub enum SpawnPattern {
  RandomOffscreen,
  Ring { radius: f32, count: usize },
  Cluster { radius: f32, count: usize },
}

#[derive(Clone)]
pub struct SpawnRule {
  pub timer: Timer,
  pub enemy: EnemyKind,
  pub pattern: SpawnPattern,
}

#[derive(Resource)]
pub struct WaveManager {
  pub idx: usize,
  pub timer: Timer,
  pub rules: Vec<SpawnRule>,
}

impl Default for WaveManager {
  fn default() -> Self {
    Self {
      idx: 0,
      timer: Timer::from_seconds(60.0, TimerMode::Once),
      rules: vec![
        SpawnRule {
          timer: Timer::from_seconds(0.5, TimerMode::Repeating),
          enemy: EnemyKind::Swarm,
          pattern: SpawnPattern::RandomOffscreen,
        },
        SpawnRule {
          timer: Timer::from_seconds(15.0, TimerMode::Repeating),
          enemy: EnemyKind::Constellation(4),
          pattern: SpawnPattern::RandomOffscreen,
        },
      ],
    }
  }
}

#[derive(Event, Clone)]
pub struct SpawnEnemy {
  pub enemy: EnemyKind,
  pub pos: Vec2,
}

fn wave_director(
  mut commands: Commands,
  mut wave: ResMut<WaveManager>,
  time: Res<Time>,
  player: Single<&Transform2D, With<Player>>,
  camera: Single<&Projection, With<PrimaryCamera>>,
) {
  if wave.timer.tick(time.delta()).just_finished() {
    return info!("Wave {} completed!", wave.idx);
  }

  let player_pos = player.into_inner().translation;
  let Projection::Orthographic(proj) = camera.into_inner() else { return };

  let offscreen_radius = proj.area.size().length() * 0.6;

  let mut rng = rand::rng();
  for rule in &mut wave.rules {
    if rule.timer.tick(time.delta()).just_finished() {
      match &rule.pattern {
        SpawnPattern::RandomOffscreen => {
          let pos = Vec2::from_angle(rng.random_range(0.0..TAU))
            * offscreen_radius
            + player_pos;
          commands.trigger(SpawnEnemy { enemy: rule.enemy, pos });
        }
        SpawnPattern::Ring { radius, count } => {
          let angle_step = TAU / *count as f32;
          for i in 0..*count {
            let pos =
              Vec2::from_angle(angle_step * i as f32) * *radius + player_pos;
            commands.trigger(SpawnEnemy { enemy: rule.enemy, pos });
          }
        }
        SpawnPattern::Cluster { radius, count } => {
          let center = Vec2::from_angle(rng.random_range(0.0..TAU))
            * offscreen_radius
            + player_pos;
          for _ in 0..*count {
            let offset = Vec2::new(
              rng.random_range(-*radius..*radius),
              rng.random_range(-*radius..*radius),
            );
            commands
              .trigger(SpawnEnemy { enemy: rule.enemy, pos: center + offset });
          }
        }
      }
    }
  }
}
