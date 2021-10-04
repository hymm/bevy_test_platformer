use crate::ground::Ground;
use crate::physics::{
    Acceleration, ColliderType, Collision, CollisionShape, CollisionType, Collisions, Hurtbox,
    PhysicsSettings, PhysicsSettingsHandle, Position, Velocity,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct PlayerRay;

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
        .insert(Acceleration(Vec2::new(0.0, 0.0)))
        .insert(Hurtbox {
            shape: CollisionShape::Rect(Vec2::new(30.0, 30.0)),
            col_type: ColliderType::Player,
        })
        .insert(Collisions(Vec::new()));

    commands
        .spawn()
        .insert(PlayerRay)
        .insert(Hurtbox {
            shape: CollisionShape::Ray(Vec2::new(0.0, -30.1)),
            col_type: ColliderType::PlayerRay,
        })
        .insert(Position(Vec2::new(0.0, 15.0)))
        .insert(Collisions(Vec::new()));
}

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Acceleration), With<Player>>,
    physics_settings: Res<Assets<PhysicsSettings>>,
    physics_settings_handle: Res<PhysicsSettingsHandle>,
) {
    let s = physics_settings
        .get(&physics_settings_handle.0)
        .expect("no physics settings found");
    let (mut v, mut a) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) && a.0.y == 0.0 {
        v.0.y = s.initial_jump_velocity;
        a.0.y = s.hold_gravity;
    }

    if keyboard_input.just_released(KeyCode::Space) {
        a.0.y = s.normal_gravity;
    }
}

pub fn player_horizontal_accel(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Acceleration), With<Player>>,
    physics_settings: Res<Assets<PhysicsSettings>>,
    physics_settings_handle: Res<PhysicsSettingsHandle>,
) {
    let s = physics_settings
        .get(&physics_settings_handle.0)
        .expect("no physics settings found");

    let (mut v, mut a) = query.single_mut();
    if keyboard_input.pressed(KeyCode::A) {
        a.0.x = -s.horizontal_a;
    } else if keyboard_input.pressed(KeyCode::D) {
        a.0.x = s.horizontal_a;
    } else if v.0.x > s.stopping_horizontal_speed {
        a.0.x = -s.friction;
    } else if v.0.x < -s.stopping_horizontal_speed {
        a.0.x = s.friction;
    } else {
        v.0.x = 0.0;
        a.0.x = 0.0;
    }
}

pub fn handle_player_collides_ground(
    mut player_q: Query<
        (
            &mut Position,
            &mut Velocity,
            &mut Acceleration,
            &Collisions,
            &Sprite,
        ),
        (With<Player>, Changed<Collisions>),
    >,
    grounds_q: Query<Entity, With<Ground>>,
) {
    for (mut p, mut v, mut a, cs, sprite) in player_q.iter_mut() {
        for collision_data in cs.0.iter() {
            if grounds_q.get(collision_data.entity).is_ok() {
                match (&collision_data.direction, &collision_data.collision_type) {
                    (
                        &Collision::Top,
                        &CollisionType::PlayerHitsGround {
                            ground_pos,
                            ground_size,
                        },
                    ) => {
                        v.0.y = 0.0;
                        a.0.y = 0.0;
                        p.0.y = ground_pos.y + ground_size.y / 2.0 + sprite.size.y / 2.0;
                    }
                    _ => {}
                }
                // TODO: remove collision event if it matches
            }
        }
    }
}

pub fn handle_player_ray_collides_ground(
    player_rays: Query<&Collisions, (With<PlayerRay>, Changed<Collisions>)>,
) {
    for (c) in player_rays.iter() {
        for collision_data in c.0.iter() {
                match(&collision_data.direction, &collision_data.collision_type) {
                    (
                        &Collision::Top,
                        &CollisionType::PlayerRayHitsGround {
                            ground_pos, ground_size
                        }
                    ) => {

                    }
                    _ => {}
                }
            
        }
    }
}
