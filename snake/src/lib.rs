use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::entities::NewEntity,
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

use crate::elevations::Elevations;

pub mod elevations;
pub mod fruit;
pub mod fruit_effect;
pub mod snake;
pub mod snake_body;
pub mod snake_death;

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
struct Playing;

#[hyperfold_engine::state]
struct GameOver;

#[hyperfold_engine::component(Singleton)]
struct Background;

#[hyperfold_engine::system(Init)]
fn create_bkgrnd(entities: &mut dyn _engine::Components, r: &Renderer) {
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

// TODO:
// Take Option<Singleton>
// insert_if_none and insert_or_replace
// Attach systems to crate

#[hyperfold_engine::component(Singleton)]
struct GameOverScreen;

components!(labels(GameOverScreen), GameOverEids);

#[hyperfold_engine::system]
fn game_over(
    _: &GameOver::OnEnter,
    entities: &mut dyn _engine::Components,
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
        GameOver::Label,
        Elevation(Elevations::GameOverScreen as u8),
        RenderComponent::new(RenderTexture::new(Some(tex))),
        Position(Rect::from_center(0.0, 0.0, W_F, W_F))
    );
}

// TODO: Attach systems to state
#[hyperfold_engine::system]
fn restart(key: &Key, game_over: Vec<GameOverEids>, events: &mut dyn _engine::Events) {
    if !game_over.is_empty() && matches!(key.0.key, SDL_KeyCode::SDLK_r) {
        events.set_state(Playing::Data);
    }
}
