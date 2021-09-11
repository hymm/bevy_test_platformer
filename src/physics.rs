use bevy::prelude::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Ballistic;
pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);
pub struct PhysicsSettings {
    gravity: f32,
}

pub fn setup_physics(mut commands: Commands) {
    commands.insert_resource(PhysicsSettings { gravity: -400.0 });
}

pub fn ballistic_physics(
    mut query: Query<&mut Velocity, With<Ballistic>>,
    physics_config: Res<PhysicsSettings>,
) {
    for mut v in query.iter_mut() {
        v.0.y += physics_config.gravity * TIME_STEP;
    }
}

pub fn apply_velocity(mut q: Query<(&mut Position, &Velocity)>) {
    for (mut p, v) in q.iter_mut() {
        p.0 += v.0 * TIME_STEP;
    }
}

pub fn update_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in q.iter_mut() {
        t.translation = p.0.extend(0.0);
    }
}