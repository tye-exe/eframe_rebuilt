use std::marker::PhantomData;

pub type AppCreator<'app> =
    Box<dyn 'app + FnOnce(&CreationContext<'_>) -> Result<Box<dyn 'app + App>, DynError>>;

pub struct CreationContext<'s> {
    lifetime: PhantomData<&'s str>,
}

impl<'s> CreationContext<'s> {
    pub fn new() -> Self {
        Self {
            lifetime: PhantomData,
        }
    }
}

pub trait App {
    /// Called each time the UI needs repainting, which may be many times per second.
    ///
    /// Put your widgets into a [`egui::SidePanel`], [`egui::TopBottomPanel`], [`egui::CentralPanel`], [`egui::Window`] or [`egui::Area`].
    ///
    /// The [`egui::Context`] can be cloned and saved if you like.
    ///
    /// To force a repaint, call [`egui::Context::request_repaint`] at any time (e.g. from another thread).
    ///
    /// This is called for the root viewport ([`egui::ViewportId::ROOT`]).
    /// Use [`egui::Context::show_viewport_deferred`] to spawn additional viewports (windows).
    /// (A "viewport" in egui means an native OS window).
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame);
}

pub struct Frame;

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Default)]
pub struct NativeOptions;
