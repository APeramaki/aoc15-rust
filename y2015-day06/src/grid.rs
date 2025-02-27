pub trait Light {
    fn new() -> Self;
    fn toggle(&mut self);
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn brightness(&self) -> u16;
}

#[derive(Clone, Copy)]
pub enum State {
    On,
    Off,
}

#[derive(Clone, Copy)]
pub struct SimpleLight {
    state: State,
}

impl Light for SimpleLight {
    fn new() -> Self {
        Self { state: State::Off }
    }
    fn toggle(&mut self) {
        *self = Self {
            state: match self.state {
                State::On => State::Off,
                State::Off => State::On,
            },
        }
    }

    fn turn_on(&mut self) {
        *self = Self { state: State::On };
    }

    fn turn_off(&mut self) {
        *self = Self { state: State::Off }
    }
    fn brightness(&self) -> u16 {
        match self.state {
            State::On => 1,
            State::Off => 0,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct DimmableLight {
    brightness: u16,
}

impl Light for DimmableLight {
    fn new() -> Self {
        Self { brightness: 0 }
    }

    fn toggle(&mut self) {
        self.brightness += 2;
    }

    fn turn_on(&mut self) {
        self.brightness += 1;
    }

    fn turn_off(&mut self) {
        self.brightness = self.brightness.saturating_sub(1);
    }

    fn brightness(&self) -> u16 {
        self.brightness
    }
}
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
pub struct Grid<T: Light> {
    lights: Vec<T>,
}

impl<T: Light + Clone> Grid<T> {
    pub fn new() -> Self {
        Self {
            lights: vec![T::new(); 1000 * 1000],
        }
    }
    pub fn apply_instruction(&mut self, action: Action, from: (u16, u16), to: (u16, u16)) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let index: usize = y as usize * 1000 + x as usize;
                match action {
                    Action::Toggle => self.lights[index].toggle(),
                    Action::TurnOn => self.lights[index].turn_on(),
                    Action::TurnOff => self.lights[index].turn_off(),
                }
            }
        }
    }
    pub fn count_on(self) -> u32 {
        self.lights
            .iter()
            .fold(0, |acc, light| acc + light.brightness() as u32)
    }
}
