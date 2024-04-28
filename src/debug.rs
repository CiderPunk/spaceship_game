use bevy::prelude::*;

use crate::{collision_detection::CollisionEvent, schedule::InGameSet, spaceship::Spaceship};
pub struct DebugPlugin;

impl Plugin for DebugPlugin{
  fn build(&self, app: &mut App){
    app.add_systems(Update, print_position.after(InGameSet::EntityUpdates));
    app.add_systems(Update, list_collisions.after(InGameSet::EntityUpdates));
  }
}


fn print_position(query:Query<(Entity, &Transform)>){
  //log entity ID and position
  for (entity, transform) in query.iter(){
    info!("Entity {:?} is at position {:?}", entity, transform.translation);
  }
}




pub fn list_collisions(
  mut collision_event_reader: EventReader<CollisionEvent>
){
  for &CollisionEvent{ 
    entity,
    collided_entity,
  } in collision_event_reader.read(){
    info!("Collision Entity {:?} collided with {:?}", entity, collided_entity );
  }

  
}