mod app_icon;
pub mod epi;
mod epi_native;
mod event_loop_context;
mod stopwatch;
mod stuff;
mod winit_app;
mod winit_integration;
mod winit_wrapper;

use egui::ViewportId;
use epi::UserEvent;
use epi::{AppCreator, NativeOptions};
use std::time::Instant;
use winit::event_loop::EventLoop;
use winit_app::GlowWinitApp;
use winit_integration::WinitApp;
use winit_wrapper::WinitAppWrapper;

pub fn run_native(
    app_name: &str,
    mut native_options: NativeOptions,
    app_creator: AppCreator<'_>,
) -> Result<(), crate::Error> {
    return with_event_loop(native_options, |event_loop, native_options| {
        // HERE
        let glow_eframe = GlowWinitApp::new(event_loop, app_name, native_options, app_creator);
        run_and_return(event_loop, glow_eframe)
    })?;
}

/// Access a thread-local event loop.
///
/// We reuse the event-loop so we can support closing and opening an eframe window
/// multiple times. This is just a limitation of winit.
fn with_event_loop<R>(
    mut native_options: NativeOptions,
    f: impl FnOnce(&mut EventLoop<UserEvent>, NativeOptions) -> R,
) -> Result<R, crate::Error> {
    use winit::event_loop::EventLoop;

    thread_local!(static EVENT_LOOP: std::cell::RefCell<Option<EventLoop<UserEvent>>> = const { std::cell::RefCell::new(None) });

    EVENT_LOOP.with(|event_loop| {
        // Since we want to reference NativeOptions when creating the EventLoop we can't
        // do that as part of the lazy thread local storage initialization and so we instead
        // create the event loop lazily here
        let mut event_loop_lock = event_loop.borrow_mut();
        let event_loop = if let Some(event_loop) = &mut *event_loop_lock {
            log::debug!("Reusing existing event loop.");
            event_loop
        } else {
            log::debug!("Creating new event loop.");
            event_loop_lock.insert(create_event_loop(&mut native_options)?)
        };
        Ok(f(event_loop, native_options))
    })
}

fn create_event_loop(
    native_options: &mut NativeOptions,
) -> Result<EventLoop<UserEvent>, crate::Error> {
    let mut builder = winit::event_loop::EventLoop::with_user_event();

    //TYE: Required for threaded test.
    if let Some(hook) = std::mem::take(&mut native_options.event_loop_builder) {
        hook(&mut builder);
    }

    // profiling::scope!("EventLoopBuilder::build");
    Ok(builder.build()?)
}

fn run_and_return(
    event_loop: &mut EventLoop<UserEvent>,
    winit_app: impl WinitApp,
) -> Result<(), crate::Error> {
    use winit::platform::run_on_demand::EventLoopExtRunOnDemand as _;

    log::trace!("Entering the winit event loop (run_app_on_demand)…");

    let mut app = WinitAppWrapper::new(winit_app, true);
    event_loop.run_app_on_demand(&mut app)?;
    log::debug!("eframe window closed");
    app.return_result
}

/// The different problems that can occur when trying to run `eframe`.
#[derive(Debug)]
pub enum Error {
    /// Something went wrong in user code when creating the app.
    AppCreation(Box<dyn std::error::Error + Send + Sync>),

    /// An error from [`winit`].
    #[cfg(not(target_arch = "wasm32"))]
    Winit(winit::error::OsError),

    /// An error from [`winit::event_loop::EventLoop`].
    #[cfg(not(target_arch = "wasm32"))]
    WinitEventLoop(winit::error::EventLoopError),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
    Glutin(glutin::error::Error),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
    NoGlutinConfigs(glutin::config::ConfigTemplate, Box<dyn std::error::Error>),

    /// An error from [`glutin`] when using [`glow`].
    #[cfg(feature = "glow")]
    OpenGL(egui_glow::PainterError),
}

impl std::error::Error for Error {}

#[cfg(not(target_arch = "wasm32"))]
impl From<winit::error::OsError> for Error {
    #[inline]
    fn from(err: winit::error::OsError) -> Self {
        Self::Winit(err)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<winit::error::EventLoopError> for Error {
    #[inline]
    fn from(err: winit::error::EventLoopError) -> Self {
        Self::WinitEventLoop(err)
    }
}

#[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
impl From<glutin::error::Error> for Error {
    #[inline]
    fn from(err: glutin::error::Error) -> Self {
        Self::Glutin(err)
    }
}

#[cfg(feature = "glow")]
impl From<egui_glow::PainterError> for Error {
    #[inline]
    fn from(err: egui_glow::PainterError) -> Self {
        Self::OpenGL(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AppCreation(err) => write!(f, "app creation error: {err}"),

            #[cfg(not(target_arch = "wasm32"))]
            Self::Winit(err) => {
                write!(f, "winit error: {err}")
            }

            #[cfg(not(target_arch = "wasm32"))]
            Self::WinitEventLoop(err) => {
                write!(f, "winit EventLoopError: {err}")
            }

            #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
            Self::Glutin(err) => {
                write!(f, "glutin error: {err}")
            }

            #[cfg(all(feature = "glow", not(target_arch = "wasm32")))]
            Self::NoGlutinConfigs(template, err) => {
                write!(
                    f,
                    "Found no glutin configs matching the template: {template:?}. Error: {err}"
                )
            }

            #[cfg(feature = "glow")]
            Self::OpenGL(err) => {
                write!(f, "egui_glow: {err}")
            }
        }
    }
}
