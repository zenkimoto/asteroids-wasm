#[macro_use] mod macros;
mod math;
mod game_object;
mod scene;
mod bullet;
mod player;
mod asteroids;
mod hud;
mod star_field;
mod game_scene;

use quicksilver::{
    input::Event,
    input::Key,
    geom::Vector,
    graphics::{VectorFont, FontRenderer},
    run, Graphics, Input, Result, Settings, Timer, Window,
};

use scene::{Scene, Transition};
use game_scene::GameScene;

fn main() {
    run(
        Settings {
            title: "Asteroids",
            ..Settings::default()
        },
        app,
    );
}

enum SceneType {
    Asteroids(GameScene),
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

    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let font48 = ttf.to_renderer(&gfx, 48.0)?;
    let font16 = ttf.to_renderer(&gfx, 16.0)?;
    let mut scenes = initialize_game_scenes(&window_size, font48, font16);

    loop {
        let scene = get_current_game_scene(&mut scenes);

        handle_input_events(&mut input, scene).await;

        update_game_scene(&mut update_timer, &mut input, scene);

        render_game_scene(&mut draw_timer, &window, &mut gfx, scene)?;

        if scene.should_transition() {
            let font48 = ttf.to_renderer(&gfx, 48.0)?;
            let font16 = ttf.to_renderer(&gfx, 16.0)?;
            handle_scene_transition(scene.get_transition(), &mut scenes, &window_size, font48, font16);
        }
    }
}

fn get_current_game_scene(scenes: &mut Vec<SceneType>) -> &mut dyn Scene {
    debug_assert!(scenes.len() > 0);

    match scenes.last_mut() {
        Some(SceneType::Asteroids(scene)) => scene,
        _ => {
            // This should not happen.  There should always be at least
            // one state in the stack so the game knows what to render.
            panic!("No states in state stack!");
        }
    }
}

fn initialize_game_scenes(window_size: &Vector, font48: FontRenderer, font16: FontRenderer) -> Vec<SceneType> {
    vec![
        SceneType::Asteroids(GameScene::new(window_size, font48, font16))
    ]
}

async fn handle_input_events(input: &mut Input, state: &mut dyn Scene) {
    while let Some(e) = input.next_event().await {
        match e {
            Event::KeyboardInput(key) if key.is_down() == false => state.key_up(key.key()),
            _ => { }
        }
    }
}

fn update_game_scene(update_timer: &mut Timer, input: &mut Input, state: &mut dyn Scene) {
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

        if input.key_down(Key::Return) {
            state.key_down(Key::Return);
        }

        state.update(input);
    }
}

fn render_game_scene(draw_timer: &mut Timer, window: &Window, gfx: &mut Graphics, state: &mut dyn Scene) -> Result<()> {
    if draw_timer.exhaust().is_some() {
        state.render(gfx)?;
        gfx.present(window)?;
    }

    Ok(())
}

fn handle_scene_transition(transition: Option<Transition>, scenes: &mut Vec<SceneType>, window_size: &Vector, font48: FontRenderer, font16: FontRenderer) {
    match transition {
        Some(Transition::Reset) => {
            scenes.pop();
            scenes.push(SceneType::Asteroids(GameScene::new(window_size, font48, font16)))
         },
        None => { }
    }
}