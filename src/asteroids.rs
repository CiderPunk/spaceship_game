use std::f32::consts::PI;
use bevy::prelude::*;
use rand::Rng;

use crate::{asset_loader::SceneAssets, collision_detection::{Collider, CollisionDamage, CollisionGroup}, health::Health, movement::{Acceleration, MovingObjectBundle, Velocity}, schedule::InGameSet, spaceship::Spaceship};


const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;

//const SPAWN_RANGE_X: Range<f32> = -25.0 .. 25.0;
//const SPAWN_RANGE_Z: Range<f32> = 0.0 .. 25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATION_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 35.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer{
  timer: Timer,
}

pub struct AsteroidPlugin;
impl Plugin for AsteroidPlugin{
  fn build(&self, app: &mut App){
    app.insert_resource(SpawnTimer{
      timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
    })
    .add_systems(
      Update,
      (
        spawn_asteroid, 
        rotate_asteroids
      ).in_set(InGameSet::EntityUpdates),
    );
  }
}

fn spawn_asteroid(
  mut commands:Commands, 
  query:Query<&GlobalTransform, With<Spaceship>>,
  mut spawn_timer: ResMut<SpawnTimer>, 
  time:Res<Time>, 
  scene_assets:Res<SceneAssets>,
){
  spawn_timer.timer.tick(time.delta());
  if !spawn_timer.timer.just_finished(){
    return;
  }

  let Ok(target_transform) = query.get_single() else{
    return;
  };

  let mut rng = rand::thread_rng();

  let angle = rng.gen_range(0.0 .. 2. * PI);
  let mut spawn_loc = Vec3::new(0.,0.,60.);
  let rotation_quat = Quat::from_axis_angle(Vec3::Y, angle);
  spawn_loc = rotation_quat.mul_vec3(spawn_loc);

  let velocity = (target_transform.translation() - spawn_loc ).normalize_or_zero() * VELOCITY_SCALAR;


  /*
  let mut random_unit_vector = 
  || Vec3::new(rng.gen_range(-1.0 .. 1.0), 0.0, rng.gen_range(-1.0 .. 1.0))
  .normalize_or_zero();
 
   let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
 */

 let acceleration = Vec3::ZERO;
/* 



   //let velocity = random_unit_vector() * VELOCITY_SCALAR;
   //let acceleration = random_unit_vector() * ACCELERATION_SCALAR;
   let velocity = Vec3::ZERO;
   let acceleration = Vec3::ZERO;
   
 */

  commands.spawn((
    MovingObjectBundle{
      velocity: Velocity::new(velocity),
      acceleration: Acceleration::new(acceleration),
      collider:Collider::new(RADIUS, CollisionGroup::Asteroid, CollisionGroup::Player | CollisionGroup::PlayerMissile),
      model: SceneBundle{
        scene:scene_assets.asteroid.clone(),
        transform:Transform::from_translation(spawn_loc),
        ..default()
      },
    }, 
    Asteroid,
    Health::new (HEALTH),
    CollisionDamage:: new (COLLISION_DAMAGE),
  ));
}


fn rotate_asteroids(
  mut query:Query<&mut Transform, With<Asteroid>>,
  time:Res<Time>
){
  for mut transform in query.iter_mut(){
    transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds())
  }
}

/*
fn handle_asteroid_collision(
  mut commands: Commands, 
  query:Query<(Entity, &Collider), With<Asteroid>>
) {
  for(entity,collider) in query.iter(){
    for &collided_entity in collider.colliding_entities.iter(){
      //asteroid collided with another asteroid
      if query.get(collided_entity).is_ok(){
        continue;
      }
      //despawn the asteroid
      commands.entity(entity).despawn_recursive();
    }
  }
}
 */