use crate::ground::Ground;
use crate::physics::{
    Ballistic, ColliderType, CollisionShape, CollisionType, Collisions, Hurtbox, PhysicsSettings, Position,
    Velocity,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
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
        .insert(Hurtbox {
            shape: CollisionShape::Rect(Vec2::new(30.0, 30.0)),
            col_type: ColliderType::Player,
        })
        .insert(Collisions(Vec::new()));
}

pub fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity, Option<&mut Ballistic>), With<Player>>,
    physics_settings: Res<PhysicsSettings>,
) {
    if let Ok((e, mut v, ballistic)) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if let None = ballistic {
                v.0.y = physics_settings.initial_jump_velocity;
                commands.entity(e).insert(Ballistic {
                    gravity: physics_settings.hold_gravity,
                });
            }
        }

        if keyboard_input.just_released(KeyCode::Space) {
            if let Some(mut ballistic) = ballistic {
                ballistic.gravity = physics_settings.normal_gravity;
            }
        }
    }
}

pub fn handle_player_collides_ground(
    mut commands: Commands,
    mut player_q: Query<
        (Entity, &mut Position, &mut Velocity, &Collisions, &Sprite),
        (With<Player>, With<Ballistic>, Changed<Collisions>),
    >,
    grounds_q: Query<Entity, With<Ground>>,
) {
    if let Ok((e, mut p, mut v, cs, sprite)) = player_q.single_mut() {
        for collision_data in cs.0.iter() {
            if grounds_q.get(collision_data.entity).is_ok() {
                match (&collision_data.direction, &collision_data.collision_type) {
                    (&Collision::Top, &CollisionType::PlayerHitsGround { ground_pos, ground_size}) => {
                        v.0.y = 0.0;
                        commands.entity(e).remove::<Ballistic>();
                        p.0.y = ground_pos.y + ground_size.y / 2.0 + sprite.size.y / 2.0;
                    }
                    _ => {}
                }
                // TODO: remove collision event if it matches
            }
        }
    }
}
