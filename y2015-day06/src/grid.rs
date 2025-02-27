#[derive(Clone, Copy)]
pub enum State {
    On,
    Off,
}

impl State {
    fn toggle(&mut self) {
        *self = match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
pub struct Grid {
    lights: Vec<State>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            lights: vec![State::Off; 1000 * 1000],
        }
    }
    pub fn apply_instruction(&mut self, action: Action, from: (u16, u16), to: (u16, u16)) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let index: usize = y as usize * 1000 + x as usize;
                match action {
                    Action::Toggle => self.lights[index].toggle(),
                    Action::TurnOn => self.lights[index] = State::On,
                    Action::TurnOff => self.lights[index] = State::Off,
                }
            }
        }
    }
    pub fn count_on(self) -> u32 {
        self.lights.iter().fold(0, |acc, light| match light {
            State::On => acc + 1,
            State::Off => acc,
        })
    }
}
