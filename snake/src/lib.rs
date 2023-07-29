use std::ptr::addr_of_mut;

use fruit::Fruit;
use fruit_effect::FruitEffect;
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
            render_data::{Fit, RenderDataBuilderTrait, RenderTexture},
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
use snake::{Snake, SpawnSnake};

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

#[hyperfold_engine::state]
struct Playing {
    dat_a: u8,
}

#[hyperfold_engine::state]
struct GameOver;

#[hyperfold_engine::component(Singleton)]
struct Background;

// TODO: Simplify getting ids
components!(labels(Snake), SnakeId);

components!(labels(Fruit), FruitIds);
components!(labels(FruitEffect), FruitEffectIds);

components!(labels(Background), BackgroundId);

#[hyperfold_engine::system]
fn start_snake(
    _: &Playing::OnEnter,
    // TODO: Simplify Option<Singleton>
    snake: Vec<SnakeId>,
    fruits: Vec<FruitIds>,
    fruit_effects: Vec<FruitEffectIds>,
    game_over: Vec<GameOverEids>,
    bkgrnd: Vec<BackgroundId>,
    trash: &mut EntityTrash,
    entities: &mut dyn _engine::AddComponent,
    events: &mut dyn _engine::AddEvent,
    camera: &mut Camera,
    r: &Renderer,
) {
    // TODO: Simplify replace entities
    // Clear entities
    if let Some(snake) = snake.first() {
        trash.0.push(*snake.eid);
    }
    trash.0.extend(fruits.into_iter().map(|f| f.eid));
    trash.0.extend(fruit_effects.into_iter().map(|f| f.eid));
    if let Some(game_over) = game_over.first() {
        trash.0.push(*game_over.eid);
    }

    camera.0.set_pos(0.0, 0.0, Align::Center, Align::Center);

    // Background grid
    if bkgrnd.is_empty() {
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
            Background,
            Elevation(Elevations::Background as u8),
            RenderComponent::new(RenderTexture::new(Some(tex))),
            Position(Rect::from(0.0, 0.0, W_F, W_F, Align::Center, Align::Center))
        );
    }

    events.new_event(SpawnSnake);
}

#[hyperfold_engine::component(Singleton)]
struct GameOverScreen;

#[hyperfold_engine::component(Singleton)]
struct GameOverText;

components!(labels(GameOverScreen || GameOverText), GameOverEids);

#[hyperfold_engine::system]
fn game_over(
    _: &GameOver::OnEnter,
    game_over: Vec<GameOverEids>,
    entities: &mut dyn _engine::AddComponent,
    r: &Renderer,
    am: &mut AssetManager,
) {
    // TODO: Simplify create if not exist
    if game_over.is_empty() {
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
        let rect = Rect::from_center(HALF_W, HALF_W, 0.0, 0.0);
        let mut font = FontData {
            w: Some(W_I / 3),
            h: None,
            sample: "Game Over!".to_string(),
            file: TIMES.to_string(),
        };
        let mut rt = RenderText::new(font.clone())
            .with_text("Game Over!")
            .with_text_color(WHITE)
            .with_dest_align(Align::Center, Align::BotRight)
            .with_dest_fit(Fit::None)
            .with_dest_rect(rect);
        rt.render_text(rect, r, am);
        tex.draw(r, &mut rt);

        // Press 'r' text
        font.w = Some(W_I / 2);
        font.sample = "Press 'r' to restart".to_string();
        let mut rt = rt
            .with_font_data(font)
            .with_text("Press 'r' to restart")
            .with_dest_align(Align::Center, Align::TopLeft);
        rt.render_text(rect, r, am);
        tex.draw(r, &mut rt);

        let e = Entity::new();
        add_components!(
            entities,
            e,
            GameOverScreen,
            Elevation(Elevations::GameOverScreen as u8),
            RenderComponent::new(RenderTexture::new(Some(tex))),
            Position(Rect::from_center(0.0, 0.0, W_F, W_F))
        );

        // let e = Entity::new();
        // add_components!(
        //     entities,
        //     e,
        //     GameOverText,
        //     Elevation(Elevations::GameOverText as u8),
        //     RenderComponent::new(),
        //     Position(Rect::from_center(0.0, 0.0, 0.0, 0.0)),
        // );
    }
}

// TODO: Simplify checking game state, singleton exist, counts of components
// TODO: Game state machine?
#[hyperfold_engine::system]
fn restart(key: &Key, game_over: Vec<GameOverEids>, events: &mut dyn _engine::AddEvent) {
    if !game_over.is_empty() && matches!(key.0.key, SDL_KeyCode::SDLK_r) {
        events.new_event(GameOver::OnExit);
        events.new_event(Playing::OnEnter);
    }
}
