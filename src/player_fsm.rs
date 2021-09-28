use emergent::prelude::*;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Action {
    None,
    Eat,
    Sleep,
}

struct IsAction(pub Action);

impl Condition<Action> for IsAction {
    fn validate(&self, memory: &Action) -> bool {
        *memory == self.0
    }
}

fn build_machine() {
    let mut machinery = MachineryBuilder::default()
    .state(
        Action::None,
        MachineryState::task(NoTask::default())
            .change(MachineryChange::new(Action::Eat, IsAction(Action::Eat)))
            .change(MachineryChange::new(Action::Sleep, IsAction(Action::Sleep))),
    )
    .state(
        Action::Eat,
        MachineryState::task(NoTask::default())
            .change(MachineryChange::new(Action::None, true)),
    )
    .state(
        Action::Sleep,
        MachineryState::task(NoTask::default())
            .change(MachineryChange::new(Action::None, true)),
    )
    .build();
}