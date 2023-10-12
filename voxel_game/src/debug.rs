use bevy::{
  prelude::*, 
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin}
};

use crate::game::states::GameState;

use self::components::*;
use self::systems::*;

pub mod components;
pub mod systems;

pub struct DebugOverlayPlugin;

impl Plugin for DebugOverlayPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::InGame), setup)
      .add_systems(Update, (
          update_fps,
          update_frame_time,
          update_entity_count,
          update_velocity,
        )
        .run_if(in_state(GameState::InGame))
      );
  }
}