#![allow(unused_imports)]

pub use core::{fmt::Debug, hash::Hash, marker::PhantomData, time::Duration};

pub use bevy::{
  audio::Volume,
  color::palettes::tailwind::*,
  diagnostic::FrameCount,
  ecs::{
    entity_disabling::Disabled,
    spawn::{SpawnIter, SpawnWith},
  },
  input::common_conditions::*,
  math::{vec2, vec3},
  platform::collections::{HashMap, HashSet},
  // gently forbid `Transform` to avoid random usages
  prelude::{Transform as _, *},
  sprite::Anchor,
  ui::{FocusPolicy, Val::*},
  window::PrimaryWindow,
};

pub use {
  bevy_asset_loader::prelude::*,
  leafwing_input_manager::{
    InputControlKind,
    prelude::{ActionState, *},
  },
  rand::prelude::*,
  std::collections::VecDeque,
};

pub use crate::{
  assets::{CreditsAssets, InteractionAssets, LevelAssets},
  core::*,
  ui::Game,
};
