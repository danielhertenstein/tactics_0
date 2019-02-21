extern crate tcod;

use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;

struct Actor {
    x: i32,
    y: i32,
}

struct GameState {
    actors: Vec<Actor>,

}

struct Renderer {
    root: Root,
}

fn main() {
    let mut renderer = initialize_rendering_engine();
    let mut game_state = initial_game_state();

    loop {
        render_system(&renderer, &game_state);
    }
}

fn initialize_rendering_engine() -> Renderer {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
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
        ]
    }
}

fn render_system(renderer: &Renderer, game_state: &GameState) {

}
