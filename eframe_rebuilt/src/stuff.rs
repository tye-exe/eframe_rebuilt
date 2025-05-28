// /// This is how your app is created.
// ///
// /// You can use the [`CreationContext`] to setup egui, restore state, setup OpenGL things, etc.
// pub type AppCreator<'app> =
//     Box<dyn 'app + FnOnce(&CreationContext<'_>) -> Result<Box<dyn 'app + App>, anyhow::Error>>;

// pub struct CreationContext<'a> {}

// /// Implement this trait to write apps that can be compiled for both web/wasm and desktop/native using [`eframe`](https://github.com/emilk/egui/tree/main/crates/eframe).
// pub trait App {
//     /// Called each time the UI needs repainting, which may be many times per second.
//     ///
//     /// Put your widgets into a [`egui::SidePanel`], [`egui::TopBottomPanel`], [`egui::CentralPanel`], [`egui::Window`] or [`egui::Area`].
//     ///
//     /// The [`egui::Context`] can be cloned and saved if you like.
//     ///
//     /// To force a repaint, call [`egui::Context::request_repaint`] at any time (e.g. from another thread).
//     ///
//     /// This is called for the root viewport ([`egui::ViewportId::ROOT`]).
//     /// Use [`egui::Context::show_viewport_deferred`] to spawn additional viewports (windows).
//     /// (A "viewport" in egui means an native OS window).
//     fn update(&mut self, ctx: &egui::Context, frame: &mut Frame);
// }

// struct Frame {}
