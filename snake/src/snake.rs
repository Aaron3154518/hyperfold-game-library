use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::entities::NewEntity,
    framework::{
        event_system::events::Key,
        physics::{PhysicsData, Position},
        render_system::{
            render_data::{Animation, RenderAsset, RenderDataBuilderTrait, RenderDataTrait},
            AssetManager, Elevation, RenderComponent, Renderer,
        },
    },
    sdl2::SDL_KeyCode,
    utils::{
        rect::{Align, PointF, Rect},
        util::AsType,
    },
};

use crate::{
    StartSnake,
    _engine::{AddComponent, AddEvent},
    elevations::Elevations,
    fruit::SpawnFruit,
    W_F,
};

#[hyperfold_engine::component(Singleton)]
struct Snake;

#[hyperfold_engine::system]
fn new_snake(
    _: &StartSnake,
    entities: &mut dyn AddComponent,
    events: &mut dyn AddEvent,
    r: &Renderer,
    am: &mut AssetManager,
) {
    let e = Entity::new();
    let anim = Animation::new(8, 150);
    add_components!(
        entities,
        e,
        Snake,
        Elevation(Elevations::Snake as u8),
        RenderComponent::new(
            RenderAsset::from_file("res/snake/snake_ss.png", r, am).with_animation(anim)
        ),
        Position(Rect::from(
            0.0,
            0.0,
            50.0,
            50.0,
            Align::Center,
            Align::Center
        )),
        PhysicsData {
            v: PointF::new(),
            a: PointF::new(),
            boundary: Some(Rect::from(0.0, 0.0, W_F, W_F, Align::Center, Align::Center))
        },
        Speed(100.0),
        anim
    );

    events.new_event(SpawnFruit);
}

#[hyperfold_engine::component]
struct Speed(f32);

components!(labels(Snake), SnakePos, pos: &'a Position);

components!(
    labels(Snake),
    SnakePhysics,
    physics: &'a mut PhysicsData,
    speed: &'a Speed,
    tex: &'a mut RenderComponent
);

#[hyperfold_engine::system]
fn move_snake(key: &Key, snake: SnakePhysics) {
    if key.0.pressed() {
        let (x, y, rot) = match key.0.key {
            SDL_KeyCode::SDLK_a => (-snake.speed.0, 0.0, 270.0),
            SDL_KeyCode::SDLK_d => (snake.speed.0, 0.0, 90.0),
            SDL_KeyCode::SDLK_w => (0.0, -snake.speed.0, 0.0),
            SDL_KeyCode::SDLK_s => (0.0, snake.speed.0, 180.0),
            _ => return,
        };

        snake.physics.v = PointF { x, y };
        snake.tex.try_mut(|tex: &mut RenderAsset| {
            tex.set_rotation(rot, None);
        });
    }
}
