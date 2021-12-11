extern crate log;
extern crate pretty_env_logger;

use core::{
    components::{Sprite, Transform2D},
    AssetManager, EngineBuilder, GameContext, Input, KeyCode, Scene,
};
use log::info;

struct MainState {}

impl MainState {
    fn new() -> Self {
        Self {}
    }
}
impl Scene for MainState {
    fn init(
        &mut self,
        _context: &mut GameContext,
        _asset_manager: &mut AssetManager,
    ) -> Result<(), ()> {
        let texture_id_1 = _asset_manager.load_sprite("assets/awesomeface.png");
        let texture_id_2 = _asset_manager.load_sprite("assets/happy-tree.png");

        let world = &mut _context.get_world();
        world.spawn((
            Sprite {
                texture_id: texture_id_1.clone(),
            },
            Transform2D {
                position: glam::vec2(0.0, 0.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 0.0,
            },
        ));
        world.spawn((
            Sprite {
                texture_id: texture_id_2,
            },
            Transform2D {
                position: glam::vec2(600.0, 100.0),
                scale: glam::vec2(300.0, 400.0),
                rotate: 45.0,
            },
        ));
        world.spawn((
            Sprite {
                texture_id: texture_id_1,
            },
            Transform2D {
                position: glam::vec2(250.0, 400.0),
                scale: glam::vec2(150.0, 200.0),
                rotate: 0.0,
            },
        ));

        Ok(())
    }

    fn input(
        &mut self,
        _event: core::Event,
        _context: &mut GameContext,
    ) -> Result<core::InputHandled, ()> {
        Ok(core::InputHandled::None)
    }

    fn update(
        &mut self,
        _input: &mut Input,
        _context: &mut GameContext,
        _dt: f32,
    ) -> Result<core::Transition, ()> {
        if _input.is_key_pressed(KeyCode::Space) {
            info!("Space Pressed")
        }

        if _input.is_key_released(KeyCode::Space) {
            info!("Space Released")
        }
        Ok(core::Transition::None)
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_title(String::from("Hello Engine"))
        .with_size(800, 600)
        .build()
        .unwrap()
        .run(MainState::new());
}
