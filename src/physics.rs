use crate::loader::NeedToLoad;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    sprite::collide_aabb::{collide, Collision},
};

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);

pub struct Acceleration(pub Vec2);

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "fae44c41-c109-446a-a48f-0d7742ab877a"]
pub struct PhysicsSettings {
    pub normal_gravity: f32,
    pub hold_gravity: f32,
    pub initial_jump_velocity: f32,
    pub horizontal_a: f32,
    pub friction: f32,
    pub stopping_horizontal_speed: f32,
}

#[derive(Default)]
pub struct PhysicsSettingsHandle(pub Handle<PhysicsSettings>);

pub fn load_physics(
    mut need_to_load: ResMut<NeedToLoad>,
    server: Res<AssetServer>,
    mut physics_settings: ResMut<PhysicsSettingsHandle>,
) {
    physics_settings.0 = server.load("settings.physics.ron");
    need_to_load
        .handles
        .push(physics_settings.0.clone_untyped());
}

pub fn update_velocities(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut v, a) in query.iter_mut() {
        v.0 += a.0 * TIME_STEP;
    }
}

pub fn update_positions(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut p, v) in q.iter_mut() {
        p.0 += v.0 * TIME_STEP;
    }
}

pub fn update_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in q.iter_mut() {
        t.translation = p.0.extend(0.0);
    }
}

#[derive(Clone, Copy)]
pub enum CollisionShape {
    Rect(Vec2),
}

pub struct CollisionData {
    pub entity: Entity,
    pub direction: Collision,
    pub collision_type: CollisionType,
}

pub enum CollisionType {
    PlayerHitsGround { ground_pos: Vec2, ground_size: Vec2 },
}

pub enum ColliderType {
    Player,
    Ground,
}

pub struct Hitbox {
    pub shape: CollisionShape,
    pub col_type: ColliderType,
}

pub struct Hurtbox {
    pub shape: CollisionShape,
    pub col_type: ColliderType,
}

pub struct Collisions(pub Vec<CollisionData>);

impl Hurtbox {
    pub fn check_collision(
        self: &Self,
        hurt_position: &Position,
        hitbox: &Hitbox,
        hitbox_position: &Position,
        hit_entity: Entity,
    ) -> Option<CollisionData> {
        match (&self.shape, &hitbox.shape) {
            (&CollisionShape::Rect(hurt_size), &CollisionShape::Rect(hit_size)) => {
                if let Some(direction) = collide(
                    hurt_position.0.extend(0.0),
                    hurt_size,
                    hitbox_position.0.extend(0.0),
                    hit_size,
                ) {
                    return match (&self.col_type, &hitbox.col_type) {
                        (&ColliderType::Player, &ColliderType::Ground) => Some(CollisionData {
                            entity: hit_entity,
                            direction,
                            collision_type: CollisionType::PlayerHitsGround {
                                ground_pos: hitbox_position.0.clone(),
                                ground_size: hit_size.clone(),
                            },
                        }),
                        _ => None,
                    };
                }
                return None;
            }
        }
    }
}

pub fn check_collisions(
    mut hurtboxes: Query<(&Hurtbox, &Position, &mut Collisions)>,
    hitboxes: Query<(Entity, &Hitbox, &Position)>,
) {
    for (hurtbox, hurt_position, mut collisions) in hurtboxes.iter_mut() {
        for (hit_entity, hitbox, hitbox_position) in hitboxes.iter() {
            if let Some(collision) =
                hurtbox.check_collision(hurt_position, hitbox, hitbox_position, hit_entity)
            {
                collisions.0.push(collision);
            }
        }
    }
}
