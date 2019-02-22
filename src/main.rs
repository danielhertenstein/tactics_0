extern crate tcod;

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, KeyCode};

const SCREEN_WIDTH: usize = 20;
const SCREEN_HEIGHT: usize = 20;

const LIMIT_FPS: i32 = 20;

struct Actor {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Tile {
    selected: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            selected: false,
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct GameState {
    actors: Vec<Actor>,
    map: Map,
}

struct Renderer {
    root: Root,
}

fn main() {
    let mut renderer = initialize_rendering_engine();
    let mut game_state = initial_game_state();

    loop {
        let input_state = capture_input_state();
        render_system(&mut renderer, &game_state);
    }
}

fn initialize_rendering_engine() -> Renderer {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Tactics-0")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    Renderer {
        root
    }
}

fn initial_game_state() -> GameState {
    let mut map = vec![vec![Tile::new(); SCREEN_HEIGHT]; SCREEN_WIDTH];
    map[0][0].selected = true;

    GameState {
        actors: vec![
            Actor { x: 0, y: 0 },
            Actor { x: 0, y: 1 },
        ],
        map,
    }
}

fn capture_input_state() -> Key {
    match input::check_for_event(input::KEY_PRESS) {
        Some((_, Event::Key(key))) => key,
        _ => Default::default(),
    }
}

fn render_system(renderer: &mut Renderer, game_state: &GameState) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let selected = game_state.map[x][y].selected;
            let color = match selected {
                true => colors::LIGHT_GREY,
                false => colors::DARKER_GREEN,
            };
            renderer.root.set_char_background(
                x as i32,
                y as i32,
                color,
                BackgroundFlag::Set
            );
        }
    }

    for actor in &game_state.actors {
        renderer.root.set_default_foreground(colors::BLUE);
        renderer.root.put_char(
            actor.x,
            actor.y,
            'A',
            BackgroundFlag::None
        );
    }

    renderer.root.flush();

    for actor in &game_state.actors {
        renderer.root.put_char(
            actor.x,
            actor.y,
            ' ',
            BackgroundFlag::None);
    }
}
