use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::RonAssetPlugin;

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

pub struct Settings {
    pub physics: Handle<PhysicsSettings>,
}

pub fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load("settings.physics.ron");
    commands.insert_resource(Settings { physics: handle });
}
pub struct PhysicsSettingsPlugin;
impl Plugin for PhysicsSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<PhysicsSettings>::new(&["physics.ron"]));
    }
}
