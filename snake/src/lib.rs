use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::entities::{EntityTrash, NewEntity},
    framework::{
        event_system::events::Key,
        physics::Position,
        render_system::{
            drawable::Canvas,
            font::{FontData, TIMES},
            render_data::{RenderDataBuilderTrait, RenderTexture},
            render_text::RenderText,
            shapes::{Rectangle, ShapeTrait},
            AssetManager, Camera, Elevation, RenderComponent, Renderer, Texture,
        },
    },
    sdl2::{SDL_Color, SDL_KeyCode},
    utils::{
        colors::{gray, WHITE},
        rect::{Align, Point, PointF, Rect},
        util::FloatMath,
    },
};
use snake::{Snake, SnakePos};

use crate::elevations::Elevations;

pub mod elevations;
pub mod fruit;
pub mod fruit_effect;
pub mod snake;
pub mod snake_body;

hyperfold_engine::game_crate!();

pub const SQUARE_W: u32 = 50;
pub const N_SQUARES: u32 = 10;

pub const W_I: u32 = SQUARE_W * N_SQUARES;
pub const W_F: f32 = W_I as f32;
pub const HALF_W: f32 = W_F / 2.0;

pub fn pos_to_square(pos: PointF, camera: &Camera) -> Point {
    Point {
        x: (pos.x - camera.0.cx() + W_F / 2.0).round_i32() / SQUARE_W as i32,
        y: (pos.y - camera.0.cy() + W_F / 2.0).round_i32() / SQUARE_W as i32,
    }
}

pub fn square_to_pos(pos: Point, camera: &Camera) -> PointF {
    PointF {
        x: SQUARE_W as f32 * (pos.x as f32 + 0.5) + camera.0.cx() - W_F / 2.0,
        y: SQUARE_W as f32 * (pos.y as f32 + 0.5) + camera.0.cy() - W_F / 2.0,
    }
}

#[hyperfold_engine::event]
struct StartSnake;

components!(SnakeHead, snake: &'a Snake);

#[hyperfold_engine::system]
fn start_snake(
    _: &StartSnake,
    snake: Vec<SnakeHead>,
    game_over: Vec<GameOverScreenEid>,
    trash: &mut EntityTrash,
    entities: &mut dyn _engine::AddComponent,
    camera: &mut Camera,
    r: &Renderer,
) {
    if let Some(snake) = snake.first() {
        trash.0.push(*snake.eid);
    }

    if let Some(game_over) = game_over.first() {
        trash.0.push(*game_over.eid);
    }

    camera.0.set_pos(0.0, 0.0, Align::Center, Align::Center);

    // Background grid
    let tex = Texture::new(r, W_I, W_I, gray(100));
    let w = SQUARE_W as f32;
    for x in (0..N_SQUARES).map(|x| x as f32 * w) {
        for y in (0..N_SQUARES).map(|y| y as f32 * w) {
            tex.draw(
                r,
                &mut Rectangle::new().set_color(gray(200)).border(
                    Rect { x, y, w, h: w },
                    -2.0,
                    false,
                ),
            );
        }
    }
    let e = Entity::new();
    add_components!(
        entities,
        e,
        Elevation(Elevations::Background as u8),
        RenderComponent::new(RenderTexture::new(Some(tex))),
        Position(Rect::from(0.0, 0.0, W_F, W_F, Align::Center, Align::Center))
    );
}

#[hyperfold_engine::event]
struct GameOver;

#[hyperfold_engine::component(Singleton)]
struct GameOverScreen;

components!(labels(GameOverScreen), GameOverScreenEid);

#[hyperfold_engine::system]
fn game_over(
    _: &GameOver,
    entities: &mut dyn _engine::AddComponent,
    r: &Renderer,
    am: &mut AssetManager,
) {
    let tex = Texture::new(
        r,
        W_I,
        W_I,
        SDL_Color {
            r: 0,
            g: 0,
            b: 0,
            a: 64,
        },
    );

    // Game over text
    let font = FontData {
        w: Some(W_I / 3),
        h: None,
        sample: "Game Over!".to_string(),
        file: TIMES.to_string(),
    };
    let mut text = RenderText::new(font)
        .with_text("Game Over!")
        .with_text_color(WHITE)
        .with_background_color(gray(128))
        .with_dest_rect(Rect::from_center(HALF_W, HALF_W, 0.0, 0.0));
    tex.draw_asset(r, am, &mut text);

    let e = Entity::new();
    add_components!(
        entities,
        e,
        GameOverScreen,
        Elevation(Elevations::Overlay as u8),
        RenderComponent::new(RenderTexture::new(Some(tex))),
        Position(Rect::from_center(0.0, 0.0, W_F, W_F))
    );
}

#[hyperfold_engine::system]
fn restart(key: &Key, game_over: Vec<GameOverScreenEid>, events: &mut dyn _engine::AddEvent) {
    if !game_over.is_empty() && matches!(key.0.key, SDL_KeyCode::SDLK_r) {
        events.new_event(StartSnake);
    }
}
