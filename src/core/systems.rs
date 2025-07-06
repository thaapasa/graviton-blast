use bevy::ecs::schedule::InternedSystemSet;
use bevy::prelude::*;

pub use camera::*;
pub use movement::*;
pub use player_actions::*;

mod camera;
mod movement;
mod player_actions;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UpdateSet {
    Planning,
    PlayerAction,
    Movement,
    PostMovement,
    CollisionDetection,
    Finalize,
}

impl UpdateSet {
    pub fn schedule() -> impl IntoScheduleConfigs<InternedSystemSet, ()> {
        (
            UpdateSet::Planning,
            UpdateSet::PlayerAction.after(UpdateSet::Planning),
            UpdateSet::Movement.after(UpdateSet::PlayerAction),
            UpdateSet::PostMovement.after(UpdateSet::Movement),
            UpdateSet::CollisionDetection.after(UpdateSet::PostMovement),
            UpdateSet::Finalize.after(UpdateSet::CollisionDetection),
        )
            .into_configs()
    }
}
