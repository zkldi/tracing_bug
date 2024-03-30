use tracing::Subscriber;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer};

fn main() -> Result<(), eframe::Error> {
    let stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env());

    let subscriber = tracing_subscriber::registry()
        .with(stdout)
        // VVVV this is the line that causes the panic!!
        // commenting this out will result in a working app.
        .with(CustomSubscriber);

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

pub struct CustomSubscriber;

impl<S: Subscriber> Layer<S> for CustomSubscriber {}
