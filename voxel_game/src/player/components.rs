use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainPlayer;

// #[derive(Component)]
// pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component, Default)]
pub enum CameraViewMode {
  #[default]
  Spectator,
}

#[derive(Component, Debug)]
pub struct PlayerCameraState {
  pub pitch: f32,
  pub yaw: f32,
  pub forward_vector: Vec3,
}

impl Default for PlayerCameraState {
  fn default() -> Self {
    Self {
      pitch: 0.,
      yaw: 0.,
      forward_vector: Vec3::ZERO,
    }
  }
}

#[derive(Component, Debug)]
pub struct PlayerMovementState {
  // pub acceleration: f32,
  pub velocity: Vec3,
  pub forward_walking_vector: Vec3,
  pub strafe_vector: Vec3,
}

impl Default for PlayerMovementState {
  fn default() -> Self {
    Self {
      // acceleration: 0.,
      velocity: Vec3::ZERO,
      forward_walking_vector: Vec3::ZERO,
      strafe_vector: Vec3::ZERO,
    }
  }
}

#[derive(Component)]
pub struct PlayerMovementConfig {
  pub move_acceleration: f32,
  pub max_move_speed: f32,
  pub friction: f32,
  pub control_enabled: bool,
}

impl Default for PlayerMovementConfig {
  fn default() -> Self {
    Self {
      move_acceleration: 160.,
      max_move_speed: 10.,
      friction: 160.,
      control_enabled: false,
    }
  }
}

#[derive(Component)]
pub struct Crosshair;