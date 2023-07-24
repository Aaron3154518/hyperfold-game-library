use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::{entities::NewEntity, events::core::Update},
    framework::{
        physics::{PhysicsData, Position},
        render_system::{
            render_data::{RenderAsset, RenderDataBuilderTrait, RenderDataTrait},
            AssetManager, Elevation, RenderComponent, Renderer,
        },
    },
    utils::{
        rect::{Align, PointF, Rect},
        timer::{Timer, TimerTrait},
        util::AsType,
    },
};

use crate::{
    _engine::AddComponent,
    elevations::Elevations,
    fruit::EatFruit,
    snake::{Direction, Snake, SnakePivotsMut},
    W_F,
};

pub const SNAKE_W: f32 = 50.0;

#[hyperfold_engine::component]
struct SnakeBody {
    pub direction: Direction,
    pub snake_idx: usize,
    pub pivot_idx: usize,
}

#[hyperfold_engine::component(Singleton)]
struct SnakeBodyAnim {
    pub timer: Timer,
    pub frame: u32,
}

#[hyperfold_engine::system]
fn new_snake_body(
    _: &EatFruit,
    bodies: Vec<SnakeBodies>,
    snake: SnakePivotsMut,
    entities: &mut dyn AddComponent,
    r: &Renderer,
    am: &mut AssetManager,
) {
    let (mut pos, direction) = match bodies
        .iter()
        .find(|body| body.body.snake_idx == snake.pivots.body_count - 1)
    {
        Some(tail) => (tail.pos.0.center(), tail.body.direction),
        None => (PointF::new(), Direction::Up),
    };

    // TODO: spawn at edge
    match direction {
        Direction::Left => pos.x += SNAKE_W,
        Direction::Right => pos.x -= SNAKE_W,
        Direction::Up => pos.y += SNAKE_W,
        Direction::Down => pos.y -= SNAKE_W,
    }

    let e = Entity::new();
    add_components!(
        entities,
        e,
        SnakeBody {
            direction,
            snake_idx: snake.pivots.body_count,
            pivot_idx: snake.pivots.pivot_offset
        },
        Elevation(Elevations::Snake as u8),
        RenderComponent::new(
            RenderAsset::from_file("res/snake/snake_body.png", r, am)
                .with_area(Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 17.0,
                    h: 17.0
                }))
                .with_rotation(direction.rotation(90.0), None)
        ),
        Position(Rect::from(
            pos.x,
            pos.y,
            SNAKE_W,
            SNAKE_W,
            Align::Center,
            Align::Center
        )),
        PhysicsData {
            v: direction.velocity(snake.speed.0),
            a: PointF::new(),
            boundary: Some(Rect::from(0.0, 0.0, W_F, W_F, Align::Center, Align::Center))
        },
    );

    snake.pivots.body_count += 1;
}

components!(
    SnakeBodies,
    body: &'a mut SnakeBody,
    pos: &'a mut Position,
    physics: &'a mut PhysicsData,
    tex: &'a mut RenderComponent
);

#[hyperfold_engine::system]
fn update_snake_bodies(_: &Update, bodies: Vec<SnakeBodies>, snake: SnakePivotsMut) {
    for SnakeBodies {
        body,
        pos,
        physics,
        tex,
        ..
    } in bodies
    {
        // TODO: crash when hitting edge
        // Invalid pivot is not any error, could mean waiting for the next pivot or no pivots
        if let Some((piv_pos, piv_dir)) = snake
            .pivots
            .pivots
            .get(body.pivot_idx - snake.pivots.pivot_offset)
        {
            let diff = match body.direction {
                Direction::Left => piv_pos.x - pos.0.cx(),
                Direction::Right => pos.0.cx() - piv_pos.x,
                Direction::Up => piv_pos.y - pos.0.cy(),
                Direction::Down => pos.0.cy() - piv_pos.y,
            };

            // Passed the pivot
            if diff >= 0.0 {
                body.pivot_idx += 1;
                body.direction = *piv_dir;

                // Keep progress passed pivot
                pos.0
                    .set_pos(piv_pos.x, piv_pos.y, Align::Center, Align::Center);
                match body.direction {
                    Direction::Left => pos.0.x -= diff,
                    Direction::Right => pos.0.x += diff,
                    Direction::Up => pos.0.y -= diff,
                    Direction::Down => pos.0.y += diff,
                };

                physics.v = piv_dir.velocity(snake.speed.0);
                tex.try_mut(|tex: &mut RenderAsset| {
                    tex.set_rotation(piv_dir.rotation(90.0), None);
                });

                // If we are the tail, remove the pivot
                if body.snake_idx == snake.pivots.body_count - 1 {
                    snake.pivots.pivots.pop_front();
                    snake.pivots.pivot_offset += 1;
                }
            }
        }
    }
}

components!(
    labels(SnakeBody && !Snake),
    SnakeBodyImgs,
    tex: &'a mut RenderComponent
);

components!(SnakeBodyAnimCS, anim: &'a mut SnakeBodyAnim);

#[hyperfold_engine::system]
fn animate_snake_bodies(
    update: &Update,
    bodies: Vec<SnakeBodyImgs>,
    SnakeBodyAnimCS { anim, .. }: SnakeBodyAnimCS,
) {
    let n = anim.timer.add_time(update.0);
    anim.frame = (anim.frame + n) % 17;
    let rect = Rect {
        x: 0.0,
        y: anim.frame as f32,
        w: 17.0,
        h: 17.0,
    };
    for body in bodies {
        body.tex
            .try_mut(|tex: &mut RenderAsset| tex.set_area(Some(rect)));
    }
}
