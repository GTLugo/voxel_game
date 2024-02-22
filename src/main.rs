#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::time::Duration;

use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin}, prelude::*, window::PresentMode, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, log::LogPlugin};
use bevy_atmosphere::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use tracing::Level;

use self::{
  world::{*, states::*, resources::VoxelPropertiesAsset}, 
  player::*, 
  game::{*, states::*}, 
  debug::*, menu::MenuPlugin
};

mod log;
mod debug;
mod player;
mod world;
mod game;
mod menu;

fn main() {
  App::new()
    .add_state::<AssetState>()
    .add_state::<WorldState>()
    .add_state::<GameState>()
    .insert_resource(Time::<Fixed>::from_seconds(1.0 / 100.0))
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          title: "Voxel Game".into(),
          resolution: (800., 450.).into(),
          present_mode: PresentMode::AutoNoVsync,
          ..default()
        }),
        ..default()
      }).set(LogPlugin {
        level: Level::INFO,
        filter: "RUST_LOG=off,voxel_game=trace".to_string(),
        update_subscriber: None,
      }),
      TomlAssetPlugin::<VoxelPropertiesAsset>::new(&["voxel.toml", "voxels.toml"]),
      TemporalAntiAliasPlugin,
      FrameTimeDiagnosticsPlugin,
      EntityCountDiagnosticsPlugin,
      AtmospherePlugin,
      DebugOverlayPlugin,
      MenuPlugin,
      PlayerPlugin,
      GamePlugin,
      WorldPlugin,
    ))
    .run();
}