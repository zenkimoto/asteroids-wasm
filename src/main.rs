use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Window,
};

pub struct Context {
    pub window: Window,
    pub input: Input,
}

impl Context {
    fn new(window: Window, input: Input) -> Self {
        Self {
            window,
            input
        }
    }
}

trait State {
    fn update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context, gfx: &mut Graphics) -> Result<()>;
}

struct GameState {

}

impl GameState {
    fn new() -> Self {
        Self { }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) {
   
    }

    fn render(&mut self, ctx: &mut Context, gfx: &mut Graphics) -> Result<()> {
        // Clear the screen to a blank, white color
        gfx.clear(Color::WHITE);
        // Paint a blue square with a red outline in the center of our screen
        // It should have a top-left of (350, 100) and a size of (150, 100)
        let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
        gfx.fill_rect(&rect, Color::GREEN);
        gfx.stroke_rect(&rect, Color::BLUE);

        Ok(())
    }
}


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

    let mut state = GameState::new();

    loop {
        while let Some(_) = context.input.next_event().await {
            // Normally we'd do some processing here
        }

        state.update(&mut context);
        state.render(&mut context, &mut gfx)?;

        gfx.present(&context.window)?;

        // And then we'd do updates and drawing here
        // When this loop ends, the window will close and the application will stop
        // If the window is closed, our application will receive a close event and terminate also
    }
}
