use std::collections::HashMap;

use bevy::{prelude::*, reflect::{TypeUuid, TypePath}};
// use bevy_asset_loader::prelude::*;
use serde::{Serialize, Deserialize};

use super::components::*;

#[derive(Resource)]
pub struct SolidChunks {
  pub chunks: HashMap<InterChunkCoordinates, Entity>,
}

#[derive(Resource)]
pub struct LiquidChunks {
  pub chunks: HashMap<InterChunkCoordinates, Entity>,
}

#[derive(Resource)]
pub struct GasChunks {
  pub chunks: HashMap<InterChunkCoordinates, Entity>,
}

// #[derive(AssetCollection, Resource)]
// pub struct VoxelAssetCollection {
//   #[asset(path = "properties.voxels.toml")]
//   pub voxel_properties_handle: Handle<VoxelPropertiesAsset>
// }

#[derive(Debug, Default, Serialize, Deserialize, TypeUuid, TypePath)]
#[uuid = "49171c1b-41dc-4e20-864f-ce14c3050379"]
pub struct VoxelPropertiesAsset {
  pub voxel: Vec<Voxel>, // these MUST match the toml names
}

#[derive(Debug, Default, Resource)]
pub struct VoxelProperties {
  pub properties: HashMap<String, Voxel>,
}