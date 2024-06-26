use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet, state::GameState};

const DESPAWN_DISTANCE: f32 = 100.0;
const DESPAWN_DISTANCE_SQUARED: f32 = DESPAWN_DISTANCE * DESPAWN_DISTANCE ;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update, 
      (
        despawn_far_away_entities,
        despawn_dead_entities,
      ).in_set(InGameSet::DespawnEntities),
    ).add_systems(OnEnter(GameState::GameOver), despawn_all_entites);
  }
}



fn despawn_far_away_entities(
  mut commands:Commands, 
  query:Query<(Entity, &GlobalTransform), With<Health>>
){
  for (entity,transform) in query.iter(){
    let distance_squared = transform.translation().distance_squared(Vec3::ZERO);
    if distance_squared > DESPAWN_DISTANCE_SQUARED{
      commands.entity(entity).despawn_recursive();
    }
  }
}
 

/*
fn despawn_far_away_entities(
  mut commands: Commands,
  query: Query<(Entity, &GlobalTransform), With<Health>>,
) {
  for (entity, transform) in query.iter() {
      let distance = transform.translation().distance(Vec3::ZERO);

      // Entity is far away from the camera's viewport.
      if distance > DESPAWN_DISTANCE {
          commands.entity(entity).despawn_recursive();
      }
  }
}
 */

fn despawn_dead_entities(mut commands:Commands, query:Query<(Entity, &Health)>){
  for (entity,health) in query.iter(){
    if health.value <= 0.0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}

fn despawn_all_entites(mut commands:Commands,  query:Query<Entity, With<Health>>){
  for entity in query.iter(){
    commands.entity(entity).despawn_recursive();
  }

}