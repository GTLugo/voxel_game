use std::collections::HashMap;

use bevy::{prelude::*, reflect::{TypePath}};
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

#[derive(Debug, Default, Resource)]
pub struct VoxelPropertiesMap {
  pub properties: HashMap<String, VoxelProperties>,
}