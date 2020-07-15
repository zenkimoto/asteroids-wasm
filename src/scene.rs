use quicksilver::{
    input::{Key},
    Graphics, Input, Result,
};

#[derive(Clone)]
pub enum Transition {
    Reset
}

pub trait Scene {
    fn update(&mut self, input: &mut Input);
    fn render(&mut self, gfx: &mut Graphics) -> Result<()>;

    fn key_down(&mut self, key: Key) {
        println!("Key Down: {:?}", key);
    }

    fn key_up(&mut self, key: Key) {
        println!("Key Up: {:?}", key);
    }

    fn should_transition(&self) -> bool {
        false
    }

    fn get_transition(&self) -> Option<Transition> {
        None
    }
}
