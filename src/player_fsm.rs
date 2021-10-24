use crate::physics::{Acceleration, PhysicsSettings};
use bevy::prelude::*;
use emergent::prelude::*;
use std::hash::Hash;

pub struct PlayerMemory<'a> {
    a: Mut<'a, Acceleration>, // TODO: replace with a system param
                              // settings: &'static PhysicsSettings,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PlayerState {
    OnGround,
    InAirPressedB,
    InAirReleasedB,
    Dead,
}

struct OnGroundTasks;
impl<'a> Task<PlayerMemory<'a>> for OnGroundTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        memory.a.0.y = 0.0;
    }
}

struct InAirPressedBTasks;
impl<'a> Task<PlayerMemory<'a>> for InAirPressedBTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        // memory.player_query.0.y = memory.settings.hold_gravity;
        memory.a.0.y = -1.0;
    }
}

struct InAirReleasedBTasks;
impl<'a> Task<PlayerMemory<'a>> for InAirReleasedBTasks {
    fn on_enter(&mut self, memory: &mut PlayerMemory) {
        // memory.player_query.0.y = memory.settings.normal_gravity;
        memory.a.0.y = -2.0;
    }
}

#[derive(Component)]
pub struct PlayerFSM<'a>(pub Machinery<PlayerMemory<'a>, PlayerState>);
impl<'a> PlayerFSM<'a> {
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
