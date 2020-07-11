#[macro_use] mod macros;
mod math;
mod game_object;
mod state;
mod bullet;
mod player;
mod asteroids;
mod hud;
mod game_state;

use quicksilver::{
    input::Event,
    input::Key,
    geom::Vector,
    graphics::{VectorFont, FontRenderer},
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
    Asteroids(GameState),
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // let window_size = window.size();
    // HACK: Quicksilver has a bug that does not return correct window size
    // when using WASM deployment.  For now we're hard coding the
    // window size.
    let window_size = v!(1024.0, 768.0);

    println!("Window Size: {:?}", window_size);  // Default: 1024.0 x 768.0

    // Load font
    let ttf = VectorFont::load("ShareTechMono-Regular.ttf").await?;
    let font72 = ttf.to_renderer(&gfx, 72.0)?;
    let font16 = ttf.to_renderer(&gfx, 16.0)?;

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut states = initialize_game_states(&window_size, font72, font16);

    loop {
        let state = get_current_game_state(&mut states);

        handle_input_events(&mut input, state).await;

        update_game_state(&mut update_timer, &mut input, state);

        render_game_state(&mut draw_timer, &window, &mut gfx, state)?;
    }
}

fn get_current_game_state(states: &mut Vec<StateType>) -> &mut dyn State {
    debug_assert!(states.len() > 0);

    match states.last_mut() {
        Some(StateType::Asteroids(state)) => state,
        _ => {
            // This should not happen.  There should always be at least
            // one state in the stack so the game knows what to render.
            panic!("No states in state stack!");
        }
    }
}

fn initialize_game_states(window_size: &Vector, font72: FontRenderer, font16: FontRenderer) -> Vec<StateType> {
    vec![
        StateType::Asteroids(GameState::new(window_size, font72, font16))
    ]
}

async fn handle_input_events(input: &mut Input, state: &mut dyn State) {
    while let Some(e) = input.next_event().await {
        match e {
            Event::KeyboardInput(key) if key.is_down() == false => state.key_up(key.key()),
            _ => { }
        }
    }
}

fn update_game_state(update_timer: &mut Timer, input: &mut Input, state: &mut dyn State) {
    // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
    while update_timer.tick() {
        if input.key_down(Key::Left) {
            state.key_down(Key::Left);
        }

        if input.key_down(Key::Right) {
            state.key_down(Key::Right);
        }

        if input.key_down(Key::Up) {
            state.key_down(Key::Up);
        }

        if input.key_down(Key::Space) {
            state.key_down(Key::Space);
        }

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
