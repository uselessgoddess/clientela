use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.add_plugins(InputManagerPlugin::<Action>::default());
}

#[derive(Actionlike, Reflect, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
  #[actionlike(DualAxis)]
  Move,
}

pub fn map() -> InputMap<Action> {
  InputMap::default().with_dual_axis(Action::Move, VirtualDPad::wasd())
}
