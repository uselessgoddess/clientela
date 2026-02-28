use crate::prelude::*;

pub fn plugin(app: &mut App) {
  app.register_type::<D>().init_resource::<D>();

  #[cfg(feature = "dev")]
  fps_overlay(app);
}

#[cfg(feature = "dev")]
fn fps_overlay(app: &mut App) {
  use bevy::{
    dev_tools::fps_overlay::{
      FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig,
    },
    text::FontSmoothing,
  };

  app.add_plugins(FpsOverlayPlugin {
    config: FpsOverlayConfig {
      text_config: TextFont {
        font_size: 24.0,
        font_smoothing: FontSmoothing::default(),
        ..default()
      },
      text_color: Color::srgb(0.1, 1.0, 0.2),
      enabled: true,
      refresh_interval: Duration::from_millis(100),
      frame_time_graph_config: FrameTimeGraphConfig {
        enabled: true,
        min_fps: 30.0,
        target_fps: 180.0,
      },
    },
  });
}

pub fn dev() -> bool {
  cfg!(feature = "dev")
}

#[derive(
  Debug, Resource, Reflect, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
)]
#[non_exhaustive]
pub enum D {
  #[default]
  None = 0,
  L1 = 1,
  L2 = 2,
}

pub fn in_debug(level: D) -> impl FnMut(Option<Res<D>>) -> bool + Clone {
  move |debug: Option<Res<D>>| {
    level <= debug.as_deref().copied().unwrap_or(D::None)
  }
}
