use hyperfold_engine::{components, ecs::entities::EntityTrash};

use crate::_engine::AddEvent;

#[hyperfold_engine::system(Init)]
fn init(events: &mut dyn AddEvent) {
    events.new_event(snake::StartSnake);
}

#[hyperfold_engine::event]
struct GameOver;

components!(AllEntities);

#[hyperfold_engine::system]
fn game_over(_: &GameOver, trash: &mut EntityTrash, entities: Vec<AllEntities>) {
    trash.0.extend(entities.into_iter().map(|e| *e.eid))
}
