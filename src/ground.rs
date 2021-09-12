use bevy::prelude::*;
use crate::physics::{Hitbox, CollisionShape, Position};
pub struct Ground;

pub fn spawn_ground(mut commands: Commands, mut material_assets: ResMut<Assets<ColorMaterial>>) {
commands
    .spawn()
    .insert_bundle(SpriteBundle {
        material: material_assets.add(Color::rgb(0.3, 0.3, 0.3).into()).clone(),
        sprite: Sprite::new(Vec2::new(240.0, 60.0)),
        transform: Transform::from_translation(Vec3::new(0.0, -30.0, 0.0)),
        ..Default::default()
    })
    .insert(Ground)
    .insert(Position(Vec2::new(0.0, -30.0)))
    .insert(Hitbox { shape: CollisionShape::Rect(Vec2::new(240.0, 60.0))});
}