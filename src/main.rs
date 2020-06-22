use quicksilver::{
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Asteroids",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, gfx: Graphics, mut input: Input) -> Result<()> {
    let window_size = window.size();

    println!("Window Size: {:?}", window_size);  // Default: 1024.0 x 768.0

    loop {
        while let Some(_) = input.next_event().await {
            // Normally we'd do some processing here
        }
        // And then we'd do updates and drawing here
        // When this loop ends, the window will close and the application will stop
        // If the window is closed, our application will receive a close event and terminate also
    }
}
