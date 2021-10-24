use bevy::asset::LoadState;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub struct NeedToLoad {
    pub handles: Vec<HandleUntyped>,
}

impl NeedToLoad {
    pub fn check_loaded(&self, server: &Res<AssetServer>) -> LoadState {
        server.get_group_load_state(self.handles.iter().map(|handle| handle.id))
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum LoaderState {
    Setup,
    Loading,
    Loaded,
}
impl Default for LoaderState {
    fn default() -> LoaderState {
        LoaderState::Setup
    }
}

pub fn loader_setup_done(mut current_state: ResMut<LoaderState>) {
    *current_state = LoaderState::Loading;
}

pub fn check_loaded(
    need_to_load: Res<NeedToLoad>,
    server: Res<AssetServer>,
    mut current_state: ResMut<LoaderState>,
) {
    match need_to_load.check_loaded(&server) {
        LoadState::Loaded => *current_state = LoaderState::Loaded,
        _ => {}
    }
}

pub fn on_enter_loading(
    current_state: ResMut<LoaderState>,
    mut prev_state: Local<LoaderState>,
) -> ShouldRun {
    let result = if *current_state == LoaderState::Loading && *prev_state != LoaderState::Loading {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    };
    *prev_state = *current_state;
    result
}
#[derive(Default)]
pub struct PreviousState(pub LoaderState);
pub fn load_state_run_criteria(
    expected_state: Local<LoaderState>,
    // mut prev_state: Local<PreviousState>,
    current_state: Res<LoaderState>,
) -> ShouldRun {
    // introduce a 1 frame delay for hard sync point for on enter transitions.
    // if prev_state.0 != *current_state {
    //     prev_state.0 = *current_state;
    //     return ShouldRun::No;
    // }
    // prev_state.0 = *current_state;

    if *current_state == *expected_state {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn init(mut commands: Commands) {
    commands.insert_resource(LoaderState::Setup);
    commands.insert_resource(NeedToLoad {
        handles: Vec::<HandleUntyped>::new(),
    });
}
