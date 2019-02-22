extern crate tcod;

use tcod::colors;
use tcod::console::*;

use game_state::GameState;

const LIMIT_FPS: i32 = 60;

pub struct Renderer {
    root: Root,
}


pub fn initialize_rendering_engine(screen_height: i32, screen_width: i32) -> Renderer {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(screen_width, screen_height)
        .title("Tactics-0")
        .init();
    tcod::system::set_fps(LIMIT_FPS);
    Renderer {
        root,
    }
}

pub fn render_system(renderer: &mut Renderer, game_state: &GameState) {
    let map_width = game_state.map.len() as i32;
    let map_height = game_state.map[0].len() as i32;
    for y in 0..map_height {
        for x in 0..map_width {
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
