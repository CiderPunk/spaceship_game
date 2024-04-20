use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet};

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
    );
  }
}

fn despawn_far_away_entities(
  mut commands:Commands, 
  query:Query<(Entity, &GlobalTransform)>
){
  for (entity,transform) in query.iter(){
    let distance = transform.translation().distance_squared(Vec3::ZERO);
    if distance > DESPAWN_DISTANCE_SQUARED{
      commands.entity(entity).despawn_recursive();
    }
  }
}


fn despawn_dead_entities(mut commands:Commands, query:Query<(Entity, &Health)>){
  for (entity,health) in query.iter(){
    if health.value <= 0.0 {
      commands.entity(entity).despawn_recursive();
    }
  }
}