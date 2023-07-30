use hyperfold_engine::{components, ecs::entities::EntityTrash};

use crate::_engine::Events;

#[hyperfold_engine::system(Init)]
fn init(events: &mut dyn Events) {
    events.set_state(snake::Playing::Data);
}

#[hyperfold_engine::event]
struct GameOver;

components!(AllEntities);

#[hyperfold_engine::system]
fn game_over(_: &GameOver, trash: &mut EntityTrash, entities: Vec<AllEntities>) {
    trash.0.extend(entities.into_iter().map(|e| *e.eid))
}
