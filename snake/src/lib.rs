use hyperfold_engine::{
    _engine::Entity,
    add_components,
    ecs::entities::NewEntity,
    framework::{
        physics::Position,
        render_system::{
            drawable::Canvas,
            render_data::RenderTexture,
            shapes::{Rectangle, ShapeTrait},
            Camera, Elevation, RenderComponent, Renderer, Texture,
        },
    },
    utils::{
        colors::gray,
        rect::{Align, Rect},
    },
};

use crate::elevations::Elevations;

pub mod elevations;
pub mod snake;

hyperfold_engine::game_crate!();

pub const SQUARE_W: u32 = 50;
pub const N_SQUARES: u32 = 10;

pub const W_I: u32 = SQUARE_W * N_SQUARES;
pub const W_F: f32 = W_I as f32;

#[hyperfold_engine::event]
struct StartSnake;

#[hyperfold_engine::system]
fn start_snake(
    _: &StartSnake,
    entities: &mut dyn _engine::AddComponent,
    camera: &mut Camera,
    r: &Renderer,
) {
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
