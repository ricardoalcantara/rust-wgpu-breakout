use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use winit::{
    event::Event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::render::renderer::Renderer;

pub struct ReadOnlyRc<T>(Rc<RefCell<T>>);
pub struct ReadWriteRc<T>(Rc<RefCell<T>>);
impl<T> ReadOnlyRc<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn clone(from: &Self) -> Self {
        ReadOnlyRc(Rc::clone(&from.0))
    }
}

impl<T> Clone for ReadOnlyRc<T> {
    fn clone(&self) -> Self {
        Self::clone(&self)
    }
}

impl<T> ReadWriteRc<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

pub enum GameLoopState<'a> {
    Input(&'a WindowEvent<'a>),
    Update,
    Render(ReadWriteRc<Renderer<'a>>),
    Wait,
}

pub struct GameWindow {
    event_loop: Option<EventLoop<()>>,
    window: Rc<RefCell<Window>>,
    renderer: Rc<RefCell<Renderer<'static>>>,
}

impl GameWindow {
    pub fn build(window_builder: WindowBuilder) -> GameWindow {
        let event_loop = EventLoop::new();
        let window = window_builder.build(&event_loop).unwrap();

        let window = Rc::new(RefCell::new(window));
        // TODO async
        let renderer = pollster::block_on(Renderer::new(&window.borrow()));
        let renderer = Rc::new(RefCell::new(renderer));

        GameWindow {
            window,
            event_loop: Some(event_loop),
            renderer,
        }
    }

    pub fn set_render_size(&mut self, render_size: glam::UVec2) {
        // TODO Removed
        // self.renderer.borrow_mut().set_render_size(render_size)
    }

    pub fn window(&self) -> ReadOnlyRc<Window> {
        ReadOnlyRc(Rc::clone(&self.window))
    }

    pub fn window_mut(&self) -> ReadWriteRc<Window> {
        ReadWriteRc(Rc::clone(&self.window))
    }

    pub fn renderer(&self) -> ReadOnlyRc<Renderer<'static>> {
        ReadOnlyRc(Rc::clone(&self.renderer))
    }

    pub fn renderer_mut(&self) -> ReadWriteRc<Renderer<'static>> {
        ReadWriteRc(Rc::clone(&self.renderer))
    }

    pub fn run<F>(mut self, mut game_loop: F)
    where
        F: FnMut(GameLoopState, &mut ControlFlow) + 'static,
    {
        let window = self.window.clone();
        let renderer = self.renderer.clone();
        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    // if window_id == self.window.window().id() =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.borrow_mut().resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            renderer.borrow_mut().resize(**new_inner_size);
                        }
                        _ => game_loop(GameLoopState::Input(event), control_flow),
                    }
                }
                Event::MainEventsCleared => {
                    game_loop(GameLoopState::Update, control_flow);
                    game_loop(GameLoopState::Render(self.renderer_mut()), control_flow);
                    // TODO put it back
                    // match renderer.borrow_mut().render() {
                    //     Ok(_) => {}
                    //     // Reconfigure the surface if lost
                    //     Err(wgpu::SurfaceError::Lost) => renderer.borrow_mut().reconfigure(),
                    //     // The system is out of memory, we should probably quit
                    //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    //     // All other errors (Outdated, Timeout) should be resolved by the next frame
                    //     Err(e) => error!("{:?}", e),
                    // }
                }
                Event::RedrawRequested(_) => {
                    // windows_id is not required for the engine
                    window.borrow().request_redraw();
                }
                Event::RedrawEventsCleared => {
                    game_loop(GameLoopState::Wait, control_flow);
                    *control_flow = ControlFlow::Poll;
                }
                _ => {}
            }
        });
    }
}
