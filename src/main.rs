use quicksilver::{run, Settings};
use asteroids_wasm::app;

fn main() {
    run(
        Settings {
            title: "Asteroids",
            ..Settings::default()
        },
        app,
    );
}
