mod fill;
mod types;

use std::cell::RefCell;

use egui::{Memory, RawInput};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    platform::run_on_demand::EventLoopExtRunOnDemand,
    window::{Window, WindowId},
};

use crate::types::NativeOptions;
pub use crate::types::{App, AppCreator, CreationContext, Frame};

// Event loop has to persist to allow for re-opening window
thread_local!(static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoop::new().unwrap()));

pub fn run_native<'a>(
    _app_name: &str,
    _native_options: NativeOptions,
    app_creator: AppCreator<'a>,
) -> Result<(), ()> {
    let mut app = app_creator(&CreationContext::new()).unwrap();

    let mut winit_app = WinitApp {
        app: app.as_mut(),
        window_id: None,
        window: None,
        ctx: create_egui_context(),
    };

    EVENT_LOOP.with_borrow_mut(|event_loop| {
        event_loop.run_app_on_demand(&mut winit_app).unwrap();
    });

    Ok(())
}

struct WinitApp<'app> {
    window_id: Option<WindowId>,
    window: Option<Window>,
    app: &'app mut dyn App,
    ctx: egui::Context,
}

impl<'app> ApplicationHandler for WinitApp<'app> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
        let window = event_loop.create_window(window_attributes).unwrap();
        self.window_id = Some(window.id());
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == WindowEvent::Destroyed && self.window_id == Some(window_id) {
            self.window_id = None;
            event_loop.exit();
            return;
        }

        // Tick the user app
        let _out = self.ctx.run(RawInput::default(), |egui_ctx| {
            self.app.update(egui_ctx, &mut Frame {});
        });

        let window = match self.window.as_mut() {
            Some(window) => window,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                fill::cleanup_window(window);
                self.window = None;
            }
            WindowEvent::RedrawRequested => {
                fill::fill_window(window);
            }
            _ => (),
        }
    }
}

/// Create the egui context
pub fn create_egui_context() -> egui::Context {
    pub const IS_DESKTOP: bool = cfg!(any(
        target_os = "freebsd",
        target_os = "linux",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "windows",
    ));

    let egui_ctx = egui::Context::default();

    egui_ctx.set_embed_viewports(!IS_DESKTOP);

    egui_ctx.options_mut(|o| {
        // eframe supports multi-pass (Context::request_discard).
        o.max_passes = 2.try_into().unwrap();
    });

    let memory = Memory::default();
    egui_ctx.memory_mut(|mem| *mem = memory);

    egui_ctx
}
