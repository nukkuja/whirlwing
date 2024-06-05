use winit::event::KeyboardInput;

#[derive(Default)]
struct Vec2i { x: i32, y: i32, }

#[derive(Default)]
pub struct Input {
    mouse_position: Vec2i,
    mouse_delta: Vec2i,
    keyboard_inputs: Vec<KeyboardInput>,
}

impl Input {
    pub fn add_keyboard_input(&mut self, input: KeyboardInput) {
        self.keyboard_inputs.push(input);
    }
}