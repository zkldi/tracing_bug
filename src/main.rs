use tracing::Subscriber;
use tracing_subscriber::{layer::SubscriberExt, Layer};

pub struct CustomSubscriber;

impl<S: Subscriber> Layer<S> for CustomSubscriber {}

fn main() -> Result<(), eframe::Error> {
    let subscriber = tracing_subscriber::registry().with(CustomSubscriber);

    tracing::subscriber::set_global_default(subscriber).expect("failed to open log channels");

    eframe::run_native(
        "Tracing Subscriber Bug",
        Default::default(),
        Box::new(|_| Box::new(App)),
    )
}

struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("close app to trigger panic");
        });
    }
}
