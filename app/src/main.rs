use std::time::Duration;

fn main() {
    println!("Hello, world!");
    env_logger::init();

    for _ in 0..2 {
        eframe_rebuilt::run_native(
            "test",
            Default::default(),
            Box::new(|_| Ok(Box::new(MyApp {}))),
        )
        .unwrap();

        std::thread::sleep(Duration::from_secs(2));
    }
}

struct MyApp {}

impl eframe_rebuilt::epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe_rebuilt::epi::Frame) {
        //
    }
}
