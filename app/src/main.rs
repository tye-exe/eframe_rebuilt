use std::time::Duration;

fn main() {
    env_logger::init();

    for _ in 0..2 {
        println!("Opened");
        single_thread_gui();
        println!("Closed");
        std::thread::sleep(Duration::from_secs(2));
    }
}

#[allow(dead_code)]
fn single_thread_gui() {
    eframe_rebuilt::run_native(
        "test",
        Default::default(),
        Box::new(|_| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}

#[allow(dead_code)]
fn multi_thread_gui() {
    std::thread::spawn(move || {
        let options = eframe_rebuilt::epi::NativeOptions {
            event_loop_builder: Some(Box::new(move |builder| {
                use winit::platform::wayland::EventLoopBuilderExtWayland;

                builder.with_any_thread(true);
            })),
            ..Default::default()
        };

        eframe_rebuilt::run_native(
            "test",
            options,
            Box::new(|_| Ok(Box::new(MyApp::default()))),
        )
        .unwrap();
    })
    .join()
    .unwrap();
}

#[derive(Default)]
struct MyApp {
    ticks: u8,
}

impl eframe_rebuilt::epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe_rebuilt::epi::Frame) {
        if self.ticks == 2 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        self.ticks += 1;
        ctx.request_repaint();
    }
}
