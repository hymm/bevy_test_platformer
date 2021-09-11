mod physics;
use crate::physics::{
    apply_velocity, ballistic_physics, setup_physics, update_translation, Ballistic, Position,
    Velocity, TIME_STEP
};
use bevy::{core::FixedTimestep, prelude::*};
struct Materials {
    player_material: Handle<ColorMaterial>,
    ground_material: Handle<ColorMaterial>,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, SystemLabel)]
enum Systems {
    UpdatePosition,
    UpdateTranslation,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Generic Platformer".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_physics)
        .add_system(player_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ballistic_physics.before(Systems::UpdatePosition))
                .with_system(
                    apply_velocity
                        .label(Systems::UpdatePosition)
                        .before(Systems::UpdateTranslation),
                ),
        )
        .add_system(update_translation.label(Systems::UpdateTranslation))
        .run();
}

struct Player;
struct Ground;

fn setup(mut commands: Commands, mut material_assets: ResMut<Assets<ColorMaterial>>) {
    let materials = Materials {
        player_material: material_assets.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ground_material: material_assets.add(Color::rgb(0.3, 0.3, 0.3).into()),
    };

    // spawn player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 15.0, 0.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(Position(Vec2::new(0.0, 15.0)));

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.ground_material.clone(),
            sprite: Sprite::new(Vec2::new(240.0, 60.0)),
            transform: Transform::from_translation(Vec3::new(0.0, -30.0, 0.0)),
            ..Default::default()
        })
        .insert(Ground);

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(materials);
}

fn player_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Velocity), (With<Player>, Without<Ballistic>)>,
) {
    if let Ok((e, mut v)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            v.0.y = 400.0;
            commands.entity(e).insert(Ballistic);
        }
    }
}
