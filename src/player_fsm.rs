use crate::physics::{Acceleration, PhysicsSettings};
use bevy::prelude::*;
use emergent::prelude::*;
use std::hash::Hash;

pub struct PlayerMemory {
    player_query: &'static mut Acceleration, // TODO: replace with a system param
    settings: &'static mut PhysicsSettings,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PlayerState {
    OnGround,
    InAirPressedB,
    InAirReleasedB,
    Dead,
}

struct OnGroundTasks;
impl Task<PlayerMemory> for OnGroundTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        memory.player_query.0.y = 0.0;
    }
}

struct InAirPressedBTasks;
impl Task<PlayerMemory> for InAirPressedBTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        memory.player_query.0.y = memory.settings.hold_gravity;
    }
}

struct InAirReleasedBTasks;
impl Task<PlayerMemory> for InAirReleasedBTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        memory.player_query.0.y = memory.settings.normal_gravity;
    }
}

#[derive(Component)]
pub struct PlayerFSM(pub Machinery<PlayerMemory, PlayerState>);
impl PlayerFSM {
    pub fn new() -> Self {
        let machinery = MachineryBuilder::default()
            .state(
                PlayerState::OnGround,
                MachineryState::task(NoTask::default())
                    .change(MachineryChange::new(PlayerState::InAirPressedB, true))
                    .change(MachineryChange::new(PlayerState::InAirReleasedB, true))
                    .change(MachineryChange::new(PlayerState::Dead, true)),
            )
            .state(
                PlayerState::InAirPressedB,
                MachineryState::task(NoTask::default())
                    .change(MachineryChange::new(PlayerState::OnGround, true))
                    .change(MachineryChange::new(PlayerState::InAirReleasedB, true))
                    .change(MachineryChange::new(PlayerState::Dead, true)),
            )
            .state(
                PlayerState::InAirReleasedB,
                MachineryState::task(NoTask::default())
                    .change(MachineryChange::new(PlayerState::OnGround, true))
                    .change(MachineryChange::new(PlayerState::InAirPressedB, true))
                    .change(MachineryChange::new(PlayerState::Dead, true)),
            )
            .state(
                PlayerState::Dead,
                MachineryState::task(NoTask::default())
                    .change(MachineryChange::new(PlayerState::OnGround, true)),
            )
            .build();

        PlayerFSM(machinery)
    }
}
