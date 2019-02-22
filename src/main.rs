extern crate tcod;

use tcod::colors;
use tcod::console::*;

const SCREEN_WIDTH: i32 = 20;
const SCREEN_HEIGHT: i32 = 20;

const LIMIT_FPS: i32 = 20;

struct Actor {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Tile;

impl Tile {
    pub fn new() -> Tile {
        Tile {}
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
    GameState {
        actors: vec![
            Actor { x: 0, y: 0 },
            Actor { x: 0, y: 1 },
        ],
        map: vec![vec![Tile::new(); SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize],
    }
}

fn render_system(renderer: &mut Renderer, game_state: &GameState) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            renderer.root.set_char_background(
                x,
                y,
                colors::GREEN,
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
