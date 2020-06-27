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

enum StateType {
    InitialState(GameState),
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
   let window_size = window.size();

    println!("Window Size: {:?}", window_size);  // Default: 1024.0 x 768.0

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut states = vec![
        StateType::InitialState(GameState::new(&window_size))
    ];

    loop {
        if let Some(state_type) = states.last_mut() {
            let state = match state_type {
                StateType::InitialState(state) => state,
            };

            handle_input_events(&mut input, state).await;

            update_game_state(&mut update_timer, &mut input, state);

            render_game_state(&mut draw_timer, &window, &mut gfx, state)?;
        } else {
            panic!("No states in state stack!");
        }
    }
}

async fn handle_input_events(input: &mut Input, state: &mut dyn State) {
    while let Some(e) = input.next_event().await {
        match e {
            Event::KeyboardInput(key) if key.is_down() => state.key_down(key.key()),
            Event::KeyboardInput(key) if key.is_down() == false => state.key_up(key.key()),
            _ => { }
        }
     }
}

fn update_game_state(update_timer: &mut Timer, input: &mut Input, state: &mut dyn State) {
    // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
    while update_timer.tick() {
        state.update(input);
    }
}

fn render_game_state(draw_timer: &mut Timer, window: &Window, gfx: &mut Graphics, state: &mut dyn State) -> Result<()> {
    if draw_timer.exhaust().is_some() {
        state.render(gfx)?;
        gfx.present(window)?;
    }

    Ok(())
}
