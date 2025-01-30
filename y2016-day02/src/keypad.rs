// I kinda hated the idea to make this part like I did the first one.
// Not sure if this is better, but it was way easier to debug atleast.
#[derive(Clone, Copy)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Key {
                Key1=1,
        Key2,   Key3,   Key4,
Key5,   Key6,   Key7,   Key8,   Key9,
        KeyA,   KeyB,   KeyC,
                KeyD
}

pub struct Keypad {
    current: Key,
}
impl Keypad {
    pub fn new() -> Self {
        Self { current: Key::Key5 }
    }
    pub fn move_finger(&mut self, dir: char) {
        use Key::*;

        self.current = match (self.current, dir) {
            (Key1, 'D') => Key3,
            (Key2, 'D') => Key6, (Key2, 'R') => Key3,
            (Key3, 'U') => Key1, (Key3, 'R') => Key4, (Key3, 'D') => Key7, (Key3, 'L') => Key2,
            (Key4, 'D') => Key8, (Key4, 'L') => Key3,
            (Key5, 'R') => Key6,
            (Key6, 'U') => Key2, (Key6, 'R') => Key7, (Key6, 'D') => KeyA, (Key6, 'L') => Key5,
            (Key7, 'U') => Key3, (Key7, 'R') => Key8, (Key7, 'D') => KeyB, (Key7, 'L') => Key6,
            (Key8, 'U') => Key4, (Key8, 'R') => Key9, (Key8, 'D') => KeyC, (Key8, 'L') => Key7,
            (Key9, 'L') => Key8,
            (KeyA, 'U') => Key6, (KeyA, 'R') => KeyB,
            (KeyB, 'U') => Key7, (KeyB, 'R') => KeyC, (KeyB, 'D') => KeyD, (KeyB, 'L') => KeyA,
            (KeyC, 'U') => Key8, (KeyC, 'L') => KeyB,
            (KeyD, 'U') => KeyB,
            // All other moves are illegal, stay put
            _ => self.current
        };
    }
    pub fn value(&self) -> u8 {
        self.current as u8
    }
}
