mod ground;
mod physics;
mod player;
use crate::ground::spawn_ground;
use crate::physics::{
    update_velocities, check_collisions, setup_physics, update_positions, update_translation,
    TIME_STEP,
};
use crate::player::{
    handle_player_collides_ground, player_horizontal_accel, player_input, spawn_player,
};
use bevy::{core::FixedTimestep, prelude::*};

#[derive(Clone, Hash, Debug, Eq, PartialEq, SystemLabel)]
enum System {
    UpdatePosition,
    UpdateTranslation,
    Collision,
    PhysicsSet,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Generic Platformer".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_ground)
        .add_startup_system(setup_physics)
        .add_system_set(
            SystemSet::new()
                .before(System::PhysicsSet)
                .with_system(player_input)
                .with_system(player_horizontal_accel),
        )
        .add_system_set(
            SystemSet::new()
                .label(System::PhysicsSet)
                .before(System::UpdateTranslation)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_velocities.before(System::UpdatePosition))
                .with_system(update_positions.label(System::UpdatePosition))
                .with_system(
                    check_collisions
                        .label(System::Collision)
                        .after(System::UpdatePosition),
                )
                .with_system(handle_player_collides_ground.after(System::Collision)),
        )
        .add_system(update_translation.label(System::UpdateTranslation))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
