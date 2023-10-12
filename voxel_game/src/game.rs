use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use self::states::*;
use self::systems::*;

pub mod states;
pub mod systems;
pub mod components;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_loading_state(
        LoadingState::new(AssetState::Loading)
          .continue_to_state(AssetState::Building)
          .on_failure_continue_to_state(AssetState::Error)
      )
      .add_loading_state(
        LoadingState::new(AssetState::Building)
          .continue_to_state(AssetState::Loaded)
      )
      // systems
      .add_systems(OnEnter(GameState::Launching), start_loading_assets_system)
      .add_systems(OnEnter(GameState::InGame), spawn_lights_system)
      .add_systems(OnEnter(AssetState::Loaded), finish_loading_assets_system)
      .add_systems(OnEnter(AssetState::Error), failed_to_load_assets_system)
      ;
  }
}