use breakout_engine::core::{
    asset_manager::AssetManager,
    engine::{EngineBuilder, EngineSettings},
    engine_context::EngineContext,
    game_context::GameContext,
    input::{Event, Input},
    scene::{InputHandled, Scene, Transition},
};

extern crate log;
extern crate pretty_env_logger;

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
        _engine: &mut EngineContext,
    ) -> Result<(), ()> {
        Ok(())
    }

    fn input(
        &mut self,
        _event: Event,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> Result<InputHandled, ()> {
        Ok(InputHandled::None)
    }

    fn update(
        &mut self,
        _dt: f32,
        _input: &mut Input,
        _context: &mut GameContext,
        _engine: &mut EngineContext,
    ) -> Result<Transition, ()> {
        Ok(Transition::None)
    }
}

fn main() {
    pretty_env_logger::init();

    EngineBuilder::new()
        .with_settings(EngineSettings::Title(String::from("Empty")))
        .with_settings(EngineSettings::WindowSize((800, 600)))
        .build()
        .unwrap()
        .run(MainState::new());
}