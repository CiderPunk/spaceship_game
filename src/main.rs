mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;
mod schedule;
mod state;
mod health;


use bevy::prelude::*;
use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;

use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DespawnPlugin;
use movement::MovementPlugin;
use schedule::SchedulePlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;
use state::StatePlugin;

fn main() {
  App::new()
    // bevy built-ins
    .insert_resource(ClearColor(Color::rgb(0.1,0.0,0.15)))
    .insert_resource(AmbientLight{
      color: Color::default(),
      brightness: 750.0,
    })
    .add_plugins(DefaultPlugins)

//user configured plugins
    .add_plugins(SchedulePlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(MovementPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(CollisionDetectionPlugin)
    .add_plugins(DespawnPlugin)
    .add_plugins(StatePlugin)
   // .add_plugins(DebugPlugin)
    .run();
}
