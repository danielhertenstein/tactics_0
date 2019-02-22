extern crate tcod;

use tcod::colors;
use tcod::console::*;

use game_state::GameState;

const LIMIT_FPS: i32 = 60;

pub struct Renderer {
    root: Root,
    map: Offscreen,
    panel: Offscreen,
}


pub fn initialize_rendering_engine(screen_height: i32, screen_width: i32, map_height: i32,
                                   map_width: i32, panel_height: i32,
                                   panel_width: i32) -> Renderer {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(screen_width, screen_height)
        .title("Tactics-0")
        .init();
    tcod::system::set_fps(LIMIT_FPS);

    Renderer {
        root,
        map: Offscreen::new(map_width, map_height),
        panel: Offscreen::new(panel_width, panel_height),
    }
}

pub fn render_system(renderer: &mut Renderer, game_state: &GameState) {
    let map_width = renderer.map.width();
    let map_height = renderer.map.height();
    for y in 0..map_height {
        for x in 0..map_width {
            renderer.map.set_char_background(
                x,
                y,
                colors::DARKER_GREEN,
                BackgroundFlag::Set
            );
        }
    }

    renderer.map.set_char_background(
        game_state.cursor.x,
        game_state.cursor.y,
        colors::LIGHT_GREY,
        BackgroundFlag::Set,
    );

    for actor in &game_state.actors {
        renderer.map.set_default_foreground(colors::BLUE);
        renderer.map.put_char(
            actor.x,
            actor.y,
            'A',
            BackgroundFlag::None
        );
    }

    blit(
        &renderer.map,
        (0, 0),
        (map_width, map_height),
        &mut renderer.root,
        (0, 0),
        1.0,
        1.0,
    );

    renderer.panel.set_default_background(colors::BLACK);
    renderer.panel.clear();

    let panel_width = renderer.panel.width();
    let panel_height = renderer.panel.height();
    blit(
        &renderer.panel,
        (0, 0),
        (panel_width, panel_height),
        &mut renderer.root,
        (0, map_height),
        1.0,
        1.0,
    );

    renderer.root.flush();

    for actor in &game_state.actors {
        renderer.map.put_char(
            actor.x,
            actor.y,
            ' ',
            BackgroundFlag::None);
    }
}
