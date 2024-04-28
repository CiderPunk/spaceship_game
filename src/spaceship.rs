
use std::f32::consts::PI;

use bevy::prelude::*;
use crate::{asset_loader::SceneAssets, collision_detection::{Collider, CollisionDamage, CollisionGroup}, health::Health, movement::{Acceleration, MovingObjectBundle, Velocity}, schedule::InGameSet, state::GameState};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0,0.0,-20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_RADIUS:f32 = 5.0;
const SPACESHIP_HEALTH: f32 = 1000.0;
const SPACESHIP_COLLISION_DAMAGE: f32 = 100.0;

const SHOOT_TIME_SECONDS:f32 = 0.3;


const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR:f32 = 4.0;
const MISSILE_RADIUS:f32 = 1.0;
const MISSILE_HEALTH: f32 = 1.0;
const MISSILE_COLLISION_DAMAGE: f32 = 500.0;




#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipShield;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Resource, Debug)]
pub struct ShootTimer{
  timer: Timer,
}


pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin{
  fn build(&self, app: &mut App){
    app
      .insert_resource(ShootTimer{ timer:Timer::from_seconds(SHOOT_TIME_SECONDS, TimerMode::Repeating),})
      .add_systems(PostStartup, spawn_spaceship)
      .add_systems(OnEnter(GameState::GameOver), spawn_spaceship)
      .add_systems(Update, (
          spaceship_movement_controls, 
          spaceship_weapon_controls,
          spaceship_shield_controls,
        )
          .chain()
          .in_set(InGameSet::UserInput)
      )
      .add_systems(Update, spaceship_destroyed.in_set(InGameSet::EntityUpdates));
      
  }
}

fn spawn_spaceship(mut commands: Commands,scene_assets: Res<SceneAssets>){
  commands.spawn((
    MovingObjectBundle {
      velocity: Velocity::new(Vec3::ZERO),
      acceleration:Acceleration::new(Vec3::ZERO),
      collider:Collider::new(SPACESHIP_RADIUS,  CollisionGroup::Player, CollisionGroup::Asteroid),
      model: SceneBundle{
        scene:scene_assets.spaceship.clone(),
        transform: Transform::from_translation(STARTING_TRANSLATION),
        ..default()
      },
    }, 
    Spaceship,
    Health::new(SPACESHIP_HEALTH),
    CollisionDamage::new(SPACESHIP_COLLISION_DAMAGE),
  ));
}

fn spaceship_movement_controls(
  mut query: Query<(&mut Transform, &mut Acceleration, &Velocity), With<Spaceship>>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,  
){
  let Ok((mut transform, mut acceleration, velocity)) = query.get_single_mut() else{ 
    return; 
  };
  let mut rotation = 0.0;
  let mut roll  = 0.0;
  let mut movement = 0.0;

  if keyboard_input.pressed(KeyCode::KeyD){
    rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
  }
  else if keyboard_input.pressed(KeyCode::KeyA){
    rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
  }

  if keyboard_input.pressed(KeyCode::KeyS){
    movement = -SPACESHIP_SPEED;
  }
  else if keyboard_input.pressed(KeyCode::KeyW){
    movement = SPACESHIP_SPEED;
  }

  if keyboard_input.pressed(KeyCode::ShiftLeft){
    roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
  }
  else if keyboard_input.pressed(KeyCode::ControlLeft){
    roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
  }
  //rotate around Y axis
  transform.rotate_y(rotation);
  //add the roll - this does nothing in the game
  transform.rotate_local_z(roll);

  acceleration.value = -transform.forward() * movement - (velocity.value * 0.8);

}

fn spaceship_weapon_controls(
  mut commands:Commands, 
  query:Query<&Transform, With<Spaceship>>,
  mut shoot_time: ResMut<ShootTimer>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  scene_assets: Res<SceneAssets>,
  time:Res<Time>,
){
  let Ok(transform) = query.get_single() else {
    return;
  };
  shoot_time.timer.tick(time.delta());
  if shoot_time.timer.just_finished() || shoot_time.timer.paused(){
      if keyboard_input.pressed(KeyCode::Space){
        shoot_time.timer.unpause();
        let mut missile_transform = Transform::from_translation(
          transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR
        );

        missile_transform.rotation = Quat::from(transform.rotation)
          .mul_quat(Quat::from_axis_angle(Vec3::Y, -PI / 2.));

        commands.spawn(( MovingObjectBundle{
          velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
          acceleration: Acceleration::new(Vec3::ZERO),
          collider:Collider::new(MISSILE_RADIUS, CollisionGroup::PlayerMissile, CollisionGroup::Asteroid),
          model: SceneBundle{ 
            scene: scene_assets.missile.clone(),
            transform: missile_transform,
            ..default()
          }
        }, 
        SpaceshipMissile,
        Health::new (MISSILE_HEALTH),
        CollisionDamage::new(MISSILE_COLLISION_DAMAGE),
      ));
    }
    else{
      shoot_time.timer.pause();

    }

  }
}

fn spaceship_shield_controls(
  mut commands:Commands,
  query:Query<Entity, With<Spaceship>>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
){
  let Ok(spaceship) = query.get_single() else{ return; };
  if keyboard_input.pressed(KeyCode::Tab){
    commands.entity(spaceship).insert(SpaceshipShield);
  }
}



fn spaceship_destroyed( 
  mut next_state: ResMut<NextState<GameState>>,
  query: Query<(), With<Spaceship>>,
){
  if query.get_single().is_err(){
    next_state.set(GameState::GameOver);
  }
}