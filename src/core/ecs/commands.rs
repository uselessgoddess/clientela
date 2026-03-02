use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub struct CommandsOf<'w, 's, T: Component> {
  commands: Commands<'w, 's>,
  parent: Single<'w, 's, Entity, With<T>>,
}

impl<'w, 's, T: Component> CommandsOf<'w, 's, T> {
  pub fn spawn(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
    let mut child = self.commands.spawn(bundle);
    child.set_parent_in_place(*self.parent);
    child
  }

  pub fn spawn_empty(&mut self) -> EntityCommands<'_> {
    let mut child = self.commands.spawn_empty();
    child.set_parent_in_place(*self.parent);
    child
  }

  pub fn into_inner(&self) -> Entity {
    *self.parent
  }
}

impl<'w, 's, T: Component> std::ops::Deref for CommandsOf<'w, 's, T> {
  type Target = Commands<'w, 's>;
  fn deref(&self) -> &Self::Target {
    &self.commands
  }
}

impl<'w, 's, T: Component> std::ops::DerefMut for CommandsOf<'w, 's, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.commands
  }
}
