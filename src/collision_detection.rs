use bevy::{prelude::*, transform::TransformSystem, utils::HashMap};
use bitmask_enum::bitmask;
use crate::{asteroids::Asteroid, health::Health, schedule::InGameSet, spaceship::{Spaceship, SpaceshipMissile}};


#[bitmask(u32)]
pub enum CollisionGroup{
  Player,
  PlayerMissile,
  Asteroid,
}


#[derive(Component,Debug)]
pub struct Collider{
  pub radius:f32,
  pub colliding_entities:Vec<Entity>,
  pub collision_group: CollisionGroup,
  pub collision_mask: CollisionGroup,
}

impl Collider {
  pub fn new(radius:f32, collision_group:CollisionGroup, collision_mask:CollisionGroup) -> Self{
    Self{
      radius,
      colliding_entities:vec![],
      collision_group,
      collision_mask,
    }
  }
}
#[derive(Component, Debug)]
pub struct CollisionDamage{
  pub amount: f32,
}

impl CollisionDamage {
  pub fn new(amount:f32) -> Self{
    Self{ amount}
  }
}



#[derive(Event, Debug)]
pub struct CollisionEvent{
  pub entity:Entity,
  pub collided_entity: Entity,
}

impl CollisionEvent {
  pub fn new(entity: Entity, collided_entity:Entity)-> Self{
    Self { 
      entity, 
      collided_entity, 
    }
  }
}


pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
  fn build(&self, app: &mut App){
    app.add_systems(
      PostUpdate,
      collision_detection.after(TransformSystem::TransformPropagate)//.in_set(InGameSet::CollisionDetection)
    )
    /*
    .add_systems(Update, 
      (
        (
          handle_collision::<Asteroid>,
          handle_collision::<Spaceship>,
          handle_collision::<SpaceshipMissile>,
        ),
        apply_collision_damage,  
      )
      .chain()
      .in_set(InGameSet::EntityUpdates),
    )
     */
    .add_systems(
        Update,   
        apply_collision_damage.in_set(InGameSet::EntityUpdates)
      )
    .add_event::<CollisionEvent>();
  }
}


fn collision_detection(
  mut collision_event_writer:EventWriter<CollisionEvent>,
  query: Query<(Entity, &GlobalTransform, &mut Collider)>){
  //let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

  //first phase detect collisions
  for (entity_a, transform_a, collider_a) in query.iter(){
    for (entity_b, transform_b, collider_b) in query.iter(){
      if entity_a != entity_b && collider_a.collision_mask.intersects(collider_b.collision_group){
        let distance = transform_a
          .translation()
          .distance(transform_b.translation());
        if distance < collider_a.radius + collider_b.radius{


          info!("Entity {:?} ({:?}) hit entity {:?}  ({:?}) distance {:?}  ", entity_a, transform_a.translation(), entity_b, transform_b.translation(), distance);


          collision_event_writer.send(CollisionEvent::new(entity_a, entity_b));
          
          /*
          colliding_entities
            .entry(entity_a)
            .or_insert_with(Vec::new)
            .push(entity_b)
             */
        }
      }
    } 
  }

  /*
  //second phase update colliders
  for (entity, _, mut collider) in query.iter_mut(){
    collider.colliding_entities.clear();
    if let Some(collisions) = colliding_entities.get(&entity){
      collider
        .colliding_entities
        .extend(collisions.iter().copied()); 
    }
  }
 */
}
/*
fn handle_collision<T: Component>(
  mut collision_event_writer:EventWriter<CollisionEvent>, 
  query:Query<(Entity, &Collider), With<T>>
) {
  for(entity,collider) in query.iter(){
    for &collided_entity in collider.colliding_entities.iter(){
      //entity collided with another entity of the same type
      if query.get(collided_entity).is_ok(){
        continue;
      }
      //Send collision event
      collision_event_writer.send(CollisionEvent::new(entity, collided_entity));
    }
  }
}
 */

pub fn apply_collision_damage(
  mut collision_event_reader: EventReader<CollisionEvent>,
  mut health_query:Query<&mut Health>,
  collision_damage_query: Query<&CollisionDamage>,
){
  for &CollisionEvent{ 
    entity,
    collided_entity,
  } in collision_event_reader.read(){

    let Ok(mut health) = health_query.get_mut(entity) else{ 
      continue;
    };
    let Ok(collision_damagae) = collision_damage_query.get(collided_entity) else{ 
      continue;
    };
    health.value -= collision_damagae.amount;
    info!("Hurt Entity {:?} new health {:?}", entity, health.value);
  }

  
}