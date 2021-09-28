mod ground;
mod loader;
mod physics;
mod player;
mod player_fsm;
use crate::ground::spawn_ground;
use crate::loader::load_state_run_criteria;
use crate::loader::LoaderState;
use crate::physics::{
    check_collisions, clean_up_collisions, load_physics, update_positions, update_translation,
    update_velocities, PhysicsSettings, PhysicsSettingsHandle, TIME_STEP,
};
use crate::player::{
    handle_player_collides_ground, player_horizontal_accel, player_input, spawn_player,
};
use bevy::{core::FixedTimestep, prelude::*};
use bevy_asset_ron::*;

#[derive(Clone, Hash, Debug, Eq, PartialEq, SystemLabel)]
enum System {
    LoaderSet,
    UpdatePosition,
    UpdateTranslation,
    Collision,
    CollisionCleanUp,
    PhysicsSet,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Generic Platformer".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RonAssetPlugin::<PhysicsSettings>::new(&["physics.ron"]))
        .init_resource::<PhysicsSettingsHandle>()
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_ground)
        .add_startup_system(loader::init)
        .add_system_set(
            SystemSet::new()
                .label("loader setup done run criteria")
                .with_run_criteria(
                    load_state_run_criteria.config(|c| c.0 = Some(LoaderState::Setup)),
                )
                .with_system(loader::loader_setup_done),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(loader::on_enter_loading)
                .with_system(load_physics),
        )
        .add_system_set(
            SystemSet::new()
                .label(System::LoaderSet)
                .with_run_criteria(
                    load_state_run_criteria.config(|c| c.0 = Some(LoaderState::Loading)),
                )
                .with_system(loader::check_loaded),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(
                    load_state_run_criteria
                        .config(|c| c.0 = Some(LoaderState::Loaded))
                        .label("player input run criteria"),
                )
                .after(System::LoaderSet)
                .before(System::PhysicsSet)
                .with_system(player_input)
                .with_system(player_horizontal_accel),
        )
        .add_system_set(
            SystemSet::new()
                .label(System::PhysicsSet)
                .with_run_criteria(
                    load_state_run_criteria
                        .config(|c| c.0 = Some(LoaderState::Loaded))
                        .label("physics set run criteria"),
                )
                .after(System::LoaderSet)
                .before(System::UpdateTranslation)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(update_velocities.before(System::UpdatePosition))
                .with_system(update_positions.label(System::UpdatePosition))
                .with_system(
                    check_collisions
                        .label(System::Collision)
                        .after(System::UpdatePosition),
                )
                .with_system(
                    handle_player_collides_ground
                        .after(System::Collision)
                        .before(System::CollisionCleanUp),
                )
                .with_system(clean_up_collisions.label(System::CollisionCleanUp)),
        )
        .add_system(update_translation.label(System::UpdateTranslation))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}
