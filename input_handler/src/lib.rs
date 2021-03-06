extern crate tcod;

use tcod::input::{self, Event, Key};

pub fn capture_input_state() -> Key {
    match input::check_for_event(input::KEY_PRESS) {
        Some((_, Event::Key(key))) => key,
        _ => Default::default(),
    }
}
