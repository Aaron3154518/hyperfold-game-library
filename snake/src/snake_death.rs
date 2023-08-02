use hyperfold_engine::{
    _engine::Entity,
    add_components, components,
    ecs::entities::NewEntity,
    framework::{
        physics::Position,
        render_system::{
            render_data::{Animation, RenderAsset, RenderDataBuilderTrait, RenderDataTrait},
            AssetManager, Elevation, RenderComponent, Renderer,
        },
    },
    utils::util::AsType,
};

use crate::{
    _engine::Components, elevations::Elevations, snake_body::SnakeBody, GameOver, Playing,
};

#[hyperfold_engine::component]
struct SnakeDeath;

components!(
    SnakeBodies,
    pos: &'a Position,
    body: &'a SnakeBody,
    tex: &'a RenderComponent
);

#[hyperfold_engine::system]
fn snake_death(
    _: &Playing::OnExit,
    bodies: Vec<SnakeBodies>,
    entities: &mut dyn Components,
    r: &Renderer,
    am: &mut AssetManager,
) {
    for SnakeBodies { pos, body, tex, .. } in bodies {
        let e = Entity::new();
        let anim = Animation::once(8, 100);
        let mut asset = RenderAsset::from_file(
            match body.snake_idx {
                0 => "res/snake/snake_death.png",
                _ => "res/snake/snake_body_death.png",
            },
            r,
            am,
        )
        .with_animation(anim);
        tex.try_as(|ra: &RenderAsset| {
            asset.set_render_options(ra.get_render_opts());
            asset.set_dest(ra.get_dest_opts());
        });
        add_components!(
            entities,
            e,
            SnakeDeath,
            GameOver::Label,
            Elevation(Elevations::Snake as u8),
            *pos,
            RenderComponent::new(asset),
            anim
        );
    }
}
