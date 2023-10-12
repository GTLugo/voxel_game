use bevy::{prelude::*, math::IVec3};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// The size of a chunk in each dimension (assuming cubes)
const CHUNK_SIZE: usize = 32;

/**
  The coordinates of the chunk in the world
*/
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InterChunkCoordinates(IVec3);

/** 
  The coordinates of the voxel in the chunk
*/
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct IntraChunkCoordinates(IVec3);

#[derive(Component)]
pub struct Chunk {
  pub position: InterChunkCoordinates,
  pub voxels: [VoxelId; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
  /** 
    This is for keeping track of "voxel entities" for special voxels like chests that need extra data components
  */
  pub entities: HashMap<IntraChunkCoordinates, Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoxelId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voxel {
  pub id: String,
  pub can_collide: bool,
  pub is_opaque: bool,
  pub phase: String,
}

pub trait VoxelEntity: Send + Sync {
  /**
    logic for what to do when a voxel of this type is created
  */
  fn on_create(&self, commands: &mut Commands, voxel_coords: InterChunkCoordinates, chunk_coords: IntraChunkCoordinates) -> Entity;

  /**
    logic for when a voxel is destroyed
  */
  fn on_destroy(&self, commands: &mut Commands, voxel_entity: Entity);
}