use emergent::prelude::*;
use std::hash::Hash;

struct TransitionToOnGroundTask;
impl Task<PlayerData> for TransitionToOnGroundTask {
    fn on_enter(&mut self, memory: &mut PlayerData) {
        
    }
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum PlayerState {
    OnGround,
    InAirPressedB,
    InAirReleasedB,
    Dead,
}
struct PlayerFSM(Machinery<PlayerData, PlayerState>);
impl PlayerFSM {
    fn new() {
        let mut machinery = MachineryBuilder::default()
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
                .change(MachineryChange::new(PlayerState::OnGround, true))
        )
        .build();
    }
}