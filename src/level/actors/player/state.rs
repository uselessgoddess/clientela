use crate::{actors::Player, prelude::*};

use super::input::Action as Input;

pub fn plugin(app: &mut App) {
  register(app)
    .add_systems(Update, input.in_set(Systems::Input))
    .add_systems(Update, movement.in_set(Systems::Update));
}

fn register(app: &mut App) -> &mut App {
  app.add_message::<Action>()
}

/// A [`Message`] written for a movement input action.
#[derive(Message, Debug, Copy, Clone)]
pub enum Action {
  Move(Vec2),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct Controller;

pub fn input(
  player: Single<&ActionState<Input>>,
  mut events: MessageWriter<Action>,
) {
  let state = player.into_inner();

  let direction = state.clamped_axis_pair(&Input::Move);

  if direction.length() > 0.1 {
    // todo! fix hardcoded dead-zone (feat:gamepad)
    events.write(Action::Move(direction));
  }
}

pub fn movement(
  mut events: MessageReader<Action>,
  player: Single<(&mut Transform2D, &super::Stats), With<Player>>,
  time: Res<Time>,
) {
  let (mut transform, stats) = player.into_inner();

  for action in events.read().copied() {
    match action {
      Action::Move(direction) => {
        transform.translation += direction * stats.speed * time.delta_secs();
      }
    }
  }
}
