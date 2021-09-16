use bevy::asset::AssetPath;
use bevy::asset::LoadState;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub struct NeedToLoad {
    pub handles: Vec<HandleUntyped>,
}

impl NeedToLoad {
    pub fn load(&mut self, server: &Res<AssetServer>, path: AssetPath) {
        let handle = server.load_untyped(path);
        self.handles.push(handle);
    }

    pub fn check_loaded(&self, server: &Res<AssetServer>) -> LoadState {
        server.get_group_load_state(self.handles.iter().map(|handle| handle.id))
    }
}

#[derive(PartialEq)]
pub enum LoaderState {
    Loading,
    Loaded,
}

pub struct CurrentState(LoaderState);

pub fn check_loaded(
    need_to_load: Res<NeedToLoad>,
    server: Res<AssetServer>,
    mut current_state: ResMut<CurrentState>,
) {
    match need_to_load.check_loaded(&server) {
        LoadState::Loaded => current_state.0 = LoaderState::Loaded,
        _ => {}
    }
}

pub fn load_state_run_criteria(
    expected_state: Local<LoaderState>,
    current_state: Res<CurrentState>,
) -> ShouldRun {
    if current_state.0 == *expected_state {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn init(mut commands: Commands) {
    commands.insert_resource(LoaderState::Loading);
    commands.insert_resource(NeedToLoad {
        handles: Vec::<HandleUntyped>::new(),
    });
}
