use crate::physics::{Ballistic, Collisions, Position, Velocity, Hurtbox, CollisionShape, PhysicsSettings};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use crate::ground::Ground;
pub struct Player;


// pub struct PlayerMaterial {
//   material: Handle<ColorMaterial>
// }

pub fn spawn_player(mut commands: Commands, mut material_assets: ResMut<Assets<ColorMaterial>>) {
    let material = material_assets.add(Color::rgb(0.7, 0.7, 0.7).into());

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: material.clone(),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(Position(Vec2::new(0.0, 15.0)))
        .insert(Hurtbox{ shape: CollisionShape::Rect(Vec2::new(30.0, 30.0))})
        .insert(Collisions(Vec::new()));
}

pub fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity), (With<Player>, Without<Ballistic>)>,
    physics_settings: Res<PhysicsSettings>
) {
    if let Ok((e, mut v)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            v.0.y = physics_settings.initial_jump_velocity;
            commands.entity(e).insert(Ballistic);
        }
    }
}

pub fn handle_player_collides_ground(
    mut commands: Commands,
    mut player_q: Query<(Entity, &Position, &mut Velocity, &Collisions), (With<Player>, With<Ballistic>, Changed<Collisions>)>,
    grounds_q: Query<Entity, With<Ground>>
) {
  if let Ok((e, _p, mut v, cs)) = player_q.single_mut() {
    for collision_data in cs.0.iter() {
      if grounds_q.get(collision_data.entity).is_ok() {
        match collision_data.direction {
          Collision::Top => {
            v.0.y = 0.0;
            commands.entity(e).remove::<Ballistic>();
            // TODO: set x to top of ground
          }
          _ => {} 
        }
        // TODO: remove collision event if it matches
      }
    }
  }
}
