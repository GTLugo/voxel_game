use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::states::AssetState;
use crate::game::states::GameState;

use self::components::*;
use self::resources::*;
use self::states::*;
use self::systems::*;

pub mod components;
pub mod resources;
pub mod states;
pub mod systems;

pub mod voxel_types;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_loading_state(
        LoadingState::new(WorldState::Loading)
          .continue_to_state(WorldState::Spawning)
      )
      .add_collection_to_loading_state::<_, VoxelAssetCollection>(AssetState::Loading)
      .add_systems(OnEnter(AssetState::Building), build_voxel_assets_system)
      .add_systems(OnEnter(GameState::InGame), 
        start_loading_world_system
          .run_if(in_state(AssetState::Loaded))
      )
      .add_systems(OnEnter(WorldState::Loading), 
        build_world_system
      )
      ;
  }
}