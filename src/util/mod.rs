use bevy::prelude::*;
use camera::spawn_camera;

pub mod camera;
pub mod element_builder;

pub fn run_utilities(
  app: &mut App
) {
  app.add_systems(
    Startup,
    (
      spawn_camera,
    )
  );
}