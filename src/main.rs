mod state;
mod game_state;

use quicksilver::{
    input::Event,
    run, Graphics, Input, Result, Settings, Timer, Window,
};

use crate::state::State;
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

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut state = GameState::new(&window_size);

    loop {
        while let Some(e) = input.next_event().await {
            match e {
                Event::KeyboardInput(key) if key.is_down() => state.key_down(key.key()),
                Event::KeyboardInput(key) if key.is_down() == false => state.key_up(key.key()),
                _ => { }
            }
         }

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            state.update(&mut input);
        }

        if draw_timer.exhaust().is_some() {
            state.render(&mut input, &mut gfx)?;
            gfx.present(&window)?;
        }
    }
}
