mod context;
mod state;
mod game_state;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Timer, Window,
};

use crate::context::Context;
use crate::game_state::GameState;

fn main() {
    run(
        Settings {
            title: "Asteroids",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
   let window_size = window.size();

    println!("Window Size: {:?}", window_size);  // Default: 1024.0 x 768.0

    let mut context = Context::new(window, input);

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut state = GameState::new();

    loop {
        while let Some(e) = context.input.next_event().await {
            println!("{:?}", e);
        }

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            state.update(&mut context);
        }

        if draw_timer.exhaust().is_some() {
            state.render(&mut context, &mut gfx)?;
            gfx.present(&context.window)?;
        }

        // And then we'd do updates and drawing here
        // When this loop ends, the window will close and the application will stop
        // If the window is closed, our application will receive a close event and terminate also
    }
}
