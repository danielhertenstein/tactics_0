extern crate tcod;

use tcod::colors;
use tcod::console::*;
use tcod::input::{self, Event, Key, KeyCode};

const SCREEN_WIDTH: i32 = 20;
const SCREEN_HEIGHT: i32 = 20;

const LIMIT_FPS: i32 = 60;

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

        player_control_system(input_state, &mut game_state);

        render_system(&mut renderer, &game_state);
    }
}

fn initialize_rendering_engine() -> Renderer {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Tactics-0")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    Renderer {
        root
    }
}

fn initial_game_state() -> GameState {
    let mut map = vec![vec![Tile::new(); SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
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

fn player_control_system(input_state: Key, game_state: &mut GameState) {
    match input_state {
        Key { code: KeyCode::Up, .. } => move_cursor(0, -1, game_state),
        Key { code: KeyCode::Down, .. } => move_cursor(0, 1, game_state),
        Key { code: KeyCode::Left, .. } => move_cursor(-1, 0, game_state),
        Key { code: KeyCode::Right, .. } => move_cursor(1, 0, game_state),
        _ => {},
    }
}

fn move_cursor(dx: i32, dy: i32, game_state: &mut GameState) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_y < 0 || new_x == SCREEN_WIDTH || new_y == SCREEN_HEIGHT {
                continue
            }

            if game_state.map[x as usize][y as usize].selected {
                game_state.map[x as usize][y as usize].selected = false;
                game_state.map[new_x as usize][new_y as usize].selected = true;
                return;
            }
        }
    }
}

fn render_system(renderer: &mut Renderer, game_state: &GameState) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let selected = game_state.map[x as usize][y as usize].selected;
            let color = match selected {
                true => colors::LIGHT_GREY,
                false => colors::DARKER_GREEN,
            };
            renderer.root.set_char_background(
                x,
                y,
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
