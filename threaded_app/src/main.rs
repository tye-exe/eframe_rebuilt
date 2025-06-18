// Code adapted from: https://github.com/emilk/egui/issues/6757#issuecomment-2980399010

fn main() {
    env_logger::init();

    loop {
        spawn_gui();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn spawn_gui() {
    std::thread::spawn(move || {
        let options = eframe_rebuilt::epi::NativeOptions {
            event_loop_builder: Some(Box::new(move |builder| {
                use winit::platform::wayland::EventLoopBuilderExtWayland;

                builder.with_any_thread(true);
            })),
            ..Default::default()
        };

        eframe_rebuilt::run_native("test", options, Box::new(|_| Ok(Box::new(MyApp {})))).unwrap();
    })
    .join()
    .unwrap();
}

struct MyApp {}

impl eframe_rebuilt::epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe_rebuilt::epi::Frame) {
        //
    }
}
