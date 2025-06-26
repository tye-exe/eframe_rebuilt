// use eframe_stripped as eframe;
// use eframe_stripped::epi::App;
// use eframe_stripped::epi::Frame;

use eframe_rebuilt as eframe;
use eframe_rebuilt::App;
use eframe_rebuilt::Frame;
use std::time::Duration;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let mut args = std::env::args();
    args.next(); // executable path

    // Which variant to run
    let which = args
        .next()
        .and_then(|arg| match arg.as_str() {
            "main" => Some(Which::MainThread),
            "spawn" => Some(Which::SpawnedThread),
            _ => None,
        })
        .unwrap_or_else(|| {
            eprintln!(r#""main" or "spawn" not specified as an arg, default to main thread."#);
            Which::default()
        });

    for _ in 0..2 {
        println!("Opened");
        match which {
            Which::MainThread => main_thread_gui(),
            Which::SpawnedThread => (),
        }
        println!("Closed");
        std::thread::sleep(Duration::from_secs(1));
    }
}

/// Whether the gui should be run on the main thread or a spawned thread.
#[derive(Default)]
enum Which {
    #[default]
    MainThread,
    SpawnedThread,
}

fn main_thread_gui() {
    eframe::run_native(
        "test",
        Default::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}

// fn spawn_thread_gui() {
//     std::thread::spawn(move || {
//         let options = eframe::epi::NativeOptions {
//             event_loop_builder: Some(Box::new(move |builder| {
//                 use winit::platform::wayland::EventLoopBuilderExtWayland;

//                 builder.with_any_thread(true);
//             })),
//             ..Default::default()
//         };

//         eframe::run_native(
//             "test",
//             options,
//             Box::new(|_| Ok(Box::new(MyApp::default()))),
//         )
//         .unwrap();
//     })
//     .join()
//     .unwrap();
// }

#[derive(Default)]
struct MyApp {
    ticks: u8,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Close the application after a few updates.
        // This should give it time to initialise.
        if self.ticks >= 2 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        if self.ticks < 2 {
            self.ticks += 1;
        }
        ctx.request_repaint();

        println!("Running");
    }
}
