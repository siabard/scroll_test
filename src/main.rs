extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform, TransformBundle};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, DisplayConfig, DrawFlat, Event, Pipeline, PngFormat, PosTex, Projection, RenderBundle,
    Sprite, Stage, Texture, TextureHandle, VirtualKeyCode, WithSpriteRender,
};

struct Example;

const WINDOW_WIDTH: i32 = 512;
const WINDOW_HEIGHT: i32 = 288;
const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32;
const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32;

//const ARENA_WIDTH: f32 = 100.0;
//const ARENA_HEIGHT: f32 = 100.0;

const BACKGROUND_IMAGE_WIDTH: f32 = 1157.0;
const BACKGROUND_IMAGE_HEIGHT: f32 = 288.0;

const BOTTOM_IMAGE_WIDTH: f32 = 1100.0;
const BOTTOM_IMAGE_HEIGHT: f32 = 16.0;

#[derive(PartialEq, Eq)]
enum BGPosition {
    Middle,
    Bottom,
}

struct Background {
    pub position: BGPosition,
    pub width: f32,
    pub height: f32,
}

impl Background {
    fn new(pos: BGPosition, w: f32, h: f32) -> Background {
        Background {
            position: pos,
            width: w,
            height: h,
        }
    }
}

impl Component for Background {
    type Storage = DenseVecStorage<Self>;
}

//카메라 생성
fn initiailize_camera(world: &mut World) {
    // 카메라를 생성함
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH * 4.0,
            ARENA_HEIGHT * 4.0,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        ))
        .build();
}

// 이미지 로딩
fn initialize_background(
    world: &mut World,
    bg_sprite: TextureHandle,
    bottom_sprite: TextureHandle,
) {
    let mut center_transform = Transform::default();
    let mut bottom_transform = Transform::default();

    let bg_sprite_info = Sprite {
        left: 0.0,
        right: BACKGROUND_IMAGE_WIDTH,
        top: 0.0,
        bottom: BACKGROUND_IMAGE_HEIGHT,
    };

    let bottom_sprite_info = Sprite {
        left: 0.0,
        right: BOTTOM_IMAGE_WIDTH,
        top: 0.0,
        bottom: BOTTOM_IMAGE_HEIGHT,
    };

    // Position 맞추기
    let y = BACKGROUND_IMAGE_HEIGHT / 2.0;

    center_transform.translation = Vector3::new(BACKGROUND_IMAGE_WIDTH / 2.0 + 200.0, y, 0.0);
    bottom_transform.translation =
        Vector3::new(BOTTOM_IMAGE_WIDTH / 2.0, BOTTOM_IMAGE_HEIGHT / 2.0, 0.0);

    // 화면 가운데
    world
        .create_entity()
        .with_sprite(
            &bg_sprite_info,
            bg_sprite,
            (BACKGROUND_IMAGE_WIDTH, BACKGROUND_IMAGE_HEIGHT),
        )
        .expect("Error on bgsprite")
        .with(Background::new(
            BGPosition::Middle,
            BACKGROUND_IMAGE_WIDTH,
            BACKGROUND_IMAGE_HEIGHT,
        ))
        .with(GlobalTransform::default())
        .with(center_transform)
        .build();

    world
        .create_entity()
        .with_sprite(
            &bottom_sprite_info,
            bottom_sprite,
            (BOTTOM_IMAGE_WIDTH, BOTTOM_IMAGE_HEIGHT),
        )
        .expect("Error on bottom sprite")
        .with(Background::new(
            BGPosition::Bottom,
            BOTTOM_IMAGE_WIDTH,
            BOTTOM_IMAGE_HEIGHT,
        ))
        .with(GlobalTransform::default())
        .with(bottom_transform)
        .build();
}

impl<'a, 'b> State<GameData<'a, 'b>> for Example {
    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Background>();

        let bg_sprite = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/background.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };

        let bottom_sprite = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/ground.png",
                PngFormat,
                Default::default(),
                (),
                &texture_storage,
            )
        };
        initiailize_camera(world);
        initialize_background(world, bg_sprite, bottom_sprite);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::build("./", Example)?.build(game_data)?;
    game.run();
    Ok(())
}
