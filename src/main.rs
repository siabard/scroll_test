extern crate amethyst;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::bundle::{Result, SystemBundle};
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::timing::Time;
use amethyst::core::transform::{GlobalTransform, Transform, TransformBundle};
use amethyst::ecs::prelude::{
    Component, DenseVecStorage, DispatcherBuilder, Join, Read, ReadStorage, System, WriteStorage,
};
use amethyst::input::{is_close_requested, is_key_down, InputBundle, InputHandler};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ColorMask, DepthMode, DisplayConfig, DrawSprite, MaterialTextureSet, Pipeline,
    PngFormat, Projection, RenderBundle, Sprite, SpriteRender, SpriteSheet, SpriteSheetHandle,
    Stage, Texture, TextureCoordinates, TextureHandle, TextureMetadata, Transparent,
    VirtualKeyCode, ALPHA,
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

const BIRD_IMAGE_WIDTH: f32 = 38.0;
const BIRD_IMAGE_HEIGHT: f32 = 24.0;

#[derive(PartialEq, Eq)]
pub enum BGPosition {
    Middle,
    Bottom,
}

/// Player Character
pub struct Bird {
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
}

impl Bird {
    fn new(w: f32, h: f32, t: f32, l: f32) -> Bird {
        Bird {
            width: w,
            height: h,
            top: t,
            left: l,
        }
    }
}

impl Component for Bird {
    type Storage = DenseVecStorage<Bird>;
}

/// 배경
pub struct Background {
    pub position: BGPosition,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub loop_position: f32,
}

impl Background {
    fn new(pos: BGPosition, w: f32, h: f32, s: f32, lp: f32) -> Background {
        Background {
            position: pos,
            width: w,
            height: h,
            speed: s,
            loop_position: lp,
        }
    }
}

impl Component for Background {
    type Storage = DenseVecStorage<Background>;
}

// loading sprite_sheet
fn load_sprite_sheet(
    world: &mut World,
    texture_id: u64,
    texture: TextureHandle,
    sprite: &Sprite,
) -> SpriteSheetHandle {
    world
        .write_resource::<MaterialTextureSet>()
        .insert(texture_id, texture);

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            SpriteSheet {
                texture_id: texture_id,
                sprites: vec![sprite.clone()],
            },
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    };

    sprite_sheet_handle
}

//카메라 생성
fn initiailize_camera(world: &mut World) {
    // 카메라를 생성함
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        ))).with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into(),
        )).build();
}

// Player 로딩
fn initialize_bird(world: &mut World, bird_sprite: TextureHandle) {
    let mut center_transform = Transform::default();

    let bird_sprite_info = Sprite {
        offsets: [0.0, 0.0],
        width: BIRD_IMAGE_WIDTH as f32,
        height: BIRD_IMAGE_HEIGHT as f32,
        tex_coords: TextureCoordinates {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        },
    };

    // y 위치 맞추기
    let y = ARENA_HEIGHT / 2.0;
    let x = ARENA_WIDTH / 2.0;

    center_transform.translation = Vector3::new(x, y, 0.5);

    let sprite_handle = load_sprite_sheet(world, 0, bird_sprite, &bird_sprite_info);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_handle.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    // 화면에 올리기
    world
        .create_entity()
        .with(sprite_render)
        .with(Bird::new(BIRD_IMAGE_WIDTH, BIRD_IMAGE_HEIGHT, y, x))
        .with(GlobalTransform::default())
        .with(center_transform)
        .with(Transparent)
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
        offsets: [0.0, 0.0],
        width: BACKGROUND_IMAGE_WIDTH as f32,
        height: BACKGROUND_IMAGE_HEIGHT as f32,
        tex_coords: TextureCoordinates {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        },
    };

    let bottom_sprite_info = Sprite {
        offsets: [0.0, 0.0],
        width: BOTTOM_IMAGE_WIDTH as f32,
        height: BOTTOM_IMAGE_HEIGHT as f32,
        tex_coords: TextureCoordinates {
            left: 0.0,
            right: 1.0,
            bottom: 0.0,
            top: 1.0,
        },
    };

    // Position 맞추기
    let y = BACKGROUND_IMAGE_HEIGHT / 2.0;

    center_transform.translation = Vector3::new(BACKGROUND_IMAGE_WIDTH / 2.0, y, 0.0);
    bottom_transform.translation =
        Vector3::new(BOTTOM_IMAGE_WIDTH / 2.0, BOTTOM_IMAGE_HEIGHT / 2.0, 0.0);

    let bg_sprite_handle = load_sprite_sheet(world, 1, bg_sprite, &bg_sprite_info);

    let bg_sprite_render = SpriteRender {
        sprite_sheet: bg_sprite_handle.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    let bottom_sprite_handle = load_sprite_sheet(world, 2, bottom_sprite, &bottom_sprite_info);

    let bottom_sprite_render = SpriteRender {
        sprite_sheet: bottom_sprite_handle.clone(),
        sprite_number: 0,
        flip_horizontal: false,
        flip_vertical: false,
    };

    // 화면 가운데
    world
        .create_entity()
        .with(bg_sprite_render)
        .with(Background::new(
            BGPosition::Middle,
            BACKGROUND_IMAGE_WIDTH,
            BACKGROUND_IMAGE_HEIGHT,
            64.0,
            412.0,
        )).with(GlobalTransform::default())
        .with(center_transform)
        .build();

    world
        .create_entity()
        .with(bottom_sprite_render)
        .with(Background::new(
            BGPosition::Bottom,
            BOTTOM_IMAGE_WIDTH,
            BOTTOM_IMAGE_HEIGHT,
            128.0,
            200.0,
        )).with(GlobalTransform::default())
        .with(bottom_transform)
        .build();
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Example {
    fn handle_event(
        &mut self,
        _: StateData<GameData>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Background>();
        world.register::<Bird>();

        let bird_sprite = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/bird.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        let bg_sprite = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "texture/background.png",
                PngFormat,
                TextureMetadata::srgb_scale(),
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
                TextureMetadata::srgb_scale(),
                (),
                &texture_storage,
            )
        };

        initiailize_camera(world);
        initialize_bird(world, bird_sprite);
        initialize_background(world, bg_sprite, bottom_sprite);
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        Trans::None
    }
}

pub struct MoveBackgroundSystem;

impl<'s> System<'s> for MoveBackgroundSystem {
    type SystemData = (
        ReadStorage<'s, Background>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (bgs, mut locals, time): Self::SystemData) {
        // Move every background with its speed and turning back when it's loop position
        for (bg, local) in (&bgs, &mut locals).join() {
            local.translation[0] -= bg.speed * time.delta_seconds();

            if local.translation[0] <= 0.0 {
                local.translation[0] = bg.loop_position;
            }
        }
    }
}

pub struct BirdSystem;
impl<'s> System<'s> for BirdSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bird>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, birds, input): Self::SystemData) {
        for (bird, transform) in (&birds, &mut transforms).join() {
            if let Some(fired) = input.action_is_down("fire") {
                if fired {
                    println!("fire emitted");
                }
            }
        }
    }
}

pub struct GlobalBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GlobalBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(MoveBackgroundSystem, "move_background_system", &[]);
        Ok(())
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let binding_path = format!("{}/resources/bindings.ron", env!("CARGO_MANIFEST_DIR"));
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(GlobalBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?.with_bundle(input_bundle)?
        .with(BirdSystem, "bird_system", &["input_system"]);
    let mut game = Application::build("./", Example)?.build(game_data)?;
    game.run();
    Ok(())
}
