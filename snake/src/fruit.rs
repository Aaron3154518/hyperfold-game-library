use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::{
        entities::{EntityTrash, NewEntity},
        events::core::Update,
    },
    framework::{
        physics::Position,
        render_system::{
            render_data::RenderAsset, AssetManager, Camera, Elevation, RenderComponent, Renderer,
        },
    },
    utils::{
        rand::{new_rng, Rng},
        rect::{Align, Point, Rect},
    },
};

use crate::{
    _engine::{Components, Events},
    elevations::Elevations,
    fruit_effect::new_fruit_effect,
    pos_to_square,
    snake::SnakePos,
    square_to_pos, Playing, N_SQUARES,
};

#[hyperfold_engine::component]
struct Fruit;

#[hyperfold_engine::event]
struct EatFruit(pub Entity);

#[hyperfold_engine::event]
struct SpawnFruit;

#[hyperfold_engine::system]
pub fn new_fruit(
    _: &SpawnFruit,
    snake: SnakePos,
    entities: &mut dyn Components,
    r: &Renderer,
    am: &mut AssetManager,
    camera: &Camera,
) {
    let snake_pos = pos_to_square(snake.pos.0.center(), camera);
    let mut rng = new_rng();
    let mut pos;
    while {
        pos = Point {
            x: rng.gen_range(0..N_SQUARES) as i32,
            y: rng.gen_range(0..N_SQUARES) as i32,
        };

        pos == snake_pos
    } {}
    let pos = square_to_pos(pos, camera);

    // Fruit
    let fruit = Entity::new();
    add_components!(
        entities,
        fruit,
        Fruit,
        Playing::Label,
        Elevation(Elevations::Fruit as u8),
        RenderComponent::new(RenderAsset::from_file("res/snake/fruit.png", r, am)),
        Position(Rect::from(
            pos.x,
            pos.y,
            25.0,
            25.0,
            Align::Center,
            Align::Center
        ))
    );

    // Fruit effect
    let img = new_rng().gen_range(0..3);
    new_fruit_effect(img, fruit, pos, entities, r, am);
}

components!(labels(Fruit), FruitPos, pos: &'a Position);

#[hyperfold_engine::system]
fn collide_fruit(
    _: &Update,
    fruits: Vec<FruitPos>,
    snake: SnakePos,
    trash: &mut EntityTrash,
    events: &mut dyn Events,
) {
    for fruit in fruits {
        if fruit.pos.0.intersects(&snake.hit_box.0) {
            trash.0.push(*fruit.eid);
            events.new_event(EatFruit(*fruit.eid));
            events.new_event(SpawnFruit);
        }
    }
}
