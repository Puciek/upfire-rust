extern crate amethyst;

use crate::components::overlay::*;
use crate::components::planets::*;
use crate::components::structures::*;

use amethyst::{
    ecs::prelude::*,
    prelude::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::Vector3,
    core::transform::Transform,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub struct MainGame;

impl SimpleState for MainGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        initialize_solar_system(world);
    }
}

pub struct MainGameGraphics;

impl SimpleState for MainGameGraphics {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let sprite_sheet_handle = load_sprite_sheet(world);
        initialise_camera(world);
        initialize_solar_system_graphics(world, sprite_sheet_handle);
        initialise_debug_overlay(world);
        initialise_debug_lines(world);
    }
}

fn initialise_debug_lines(world: &mut World) {
    let mut debug_lines_component = DebugLinesComponent::with_capacity(100);
    debug_lines_component.add_direction(
        [-200.0, 0.0, 0.0].into(),
        [400.0, 0.0, 0.0].into(),
        Srgba::new(1.0, 1.0, 1.0, 1.0),
    );
    world.create_entity().with(debug_lines_component).build();
}

fn initialise_debug_overlay(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/consolas.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let looking_at_transform = UiTransform::new(
        "looking_at".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -2.,
        1.,
        50.,
        14.,
    );
    let delta_time_transform = UiTransform::new(
        "delta_time".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -20.,
        1.,
        100.,
        14.,
    );
    let time_scale_transform = UiTransform::new(
        "time_scale".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -40.,
        1.,
        30.,
        14.,
    );

    let looking_at = world
        .create_entity()
        .with(looking_at_transform)
        .with(UiText::new(
            font.clone(),
            "Sol".to_string(),
            [1., 1., 1., 1.],
            14.,
        ))
        .build();

    let delta_time = world
        .create_entity()
        .with(delta_time_transform)
        .with(UiText::new(
            font.clone(),
            "0.0".to_string(),
            [1., 1., 1., 1.],
            14.,
        ))
        .build();

    let time_scale = world
        .create_entity()
        .with(time_scale_transform)
        .with(UiText::new(font, "0 ms".to_string(), [1., 1., 1., 1.], 14.))
        .build();

    world.insert(DebugOverlayText {
        looking_at,
        delta_time,
        time_scale,
    })
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/planets.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/planets.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_solar_system_graphics(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 4,
    };

    let mut earth_transform = Transform::default();

    earth_transform.set_translation_xyz(-50.0, -50.0, 0.0);
    earth_transform.set_scale(Vector3::new(0.15, 0.15, 0.1));

    let _earth_entity = world
        .create_entity()
        .with(Deposits {
            natural: vec![Deposit {
                deposit_type: 1,
                amount: 15.0,
                difficulty: 0,
            }],
        })
        .with(Mines {
            manned: vec![
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
            ],
        })
        .with(Position {
            x: 0.0,
            y: 0.0,
            r: 200.0,
            angle: 0.0,
        })
        .with(sprite_render)
        .with(earth_transform)
        .with(Velocity { angle: 0.2 })
        .build();
}

fn initialize_solar_system(world: &mut World) {
    let _earth_entity = world
        .create_entity()
        .with(Deposits {
            natural: vec![Deposit {
                deposit_type: 1,
                amount: 15.0,
                difficulty: 0,
            }],
        })
        .with(Mines {
            manned: vec![
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
                Manned {
                    efficiency: 2.0,
                    input_type: 1,
                    output_type: 11,
                    capacity: 0.0,
                    capacity_max: 100.0,
                },
            ],
        })
        .with(Position {
            x: 0.0,
            y: 0.0,
            r: 200.0,
            angle: 0.0,
        })
        .with(Velocity { angle: 0.2 })
        .build();
}
