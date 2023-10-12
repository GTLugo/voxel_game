use std::collections::HashMap;

use bevy::prelude::*;

use super::{components::*, resources::*, states::*};

// pub fn load_voxel_assets_system(
//   mut commands: Commands, 
//   // mut meshes: ResMut<Assets<Mesh>>,
//   // mut materials: ResMut<Assets<ColorMaterial>>,
//   asset_server: Res<AssetServer>,
//   mut voxel_properties: ResMut<Assets<VoxelProperties>>,
// ) {
//   let voxel_properties = VoxelProperties(asset_server.load("properties.voxels.toml"));
// }

pub fn start_loading_world_system(
  world_state: Res<State<WorldState>>,
  mut next_world_state: ResMut<NextState<WorldState>>,
) {
  if let WorldState::Unloaded = world_state.get() {
    info!("Loading world...");
    next_world_state.set(WorldState::Loading);
  }
}

pub fn build_voxel_assets_system(
  mut commands: Commands, 
  voxel_assets: Res<VoxelAssetCollection>,
  voxel_properties_assets: Res<Assets<VoxelPropertiesAsset>>,
) {
  info!("Building assets...");
  if let Some(voxel_properties) = voxel_properties_assets.get(&voxel_assets.voxel_properties_handle) {
    // construct the hashmap
    let properties: HashMap<String, Voxel> = voxel_properties.voxel.clone().into_iter()
      .map(|p| (p.id.clone(), p))
      .collect();
  
    commands.insert_resource(VoxelProperties {
      properties
    });
  }
  
  // if let Some(voxel_properties) = voxel_properties.remove(voxel_properties_handle.0.id()) {
  //   let x = voxel_properties.voxels.get("core::air");
  // }
}

pub fn build_world_system(
  mut commands: Commands,
  voxel_properties: Res<VoxelProperties>,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<ColorMaterial>>,
) {
  info!("Building world...");
  debug!("{voxel_properties:?}");
}

// fn load_voxel_properties() -> HashMap<String, VoxelPropertiesHandle> {
//   // read the file to a string
//   let data = std::fs::read_to_string("voxel_properties.toml").unwrap();
//   // parse the string
//   let holder: VoxelProperties = toml::from_str(&data).unwrap();

//   // construct the hashmap
//   let mut voxel_properties = HashMap::new();
//   for voxel in holder.voxels {
//       voxel_properties.insert(voxel.id.clone(), voxel);
//   }

//   voxel_properties
// }