extern crate tcod;

use tcod::chars;
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

    if let Some(actor) = game_state.actors
        .iter()
        .find(|actor| actor.selected) {

        renderer.panel.print_ex(
            1,
            1,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("{}", actor.name)
        );
        if let Some(menu) = &game_state.current_menu {
            for (i, option) in menu.iter().enumerate() {
                // TODO: This unwrap feels bad. I know there should be a current menu option if the
                // current menu is not None, but I may mess up.
                if game_state.current_menu_option.unwrap() == i {
                    renderer.panel.set_char(
                        panel_width / 2,
                        i as i32 + 1,
                        chars::ARROW_E,
                    );
                    renderer.panel.print_ex(
                        panel_width / 2 + 1,
                        i as i32 + 1,
                        BackgroundFlag::None,
                        TextAlignment::Left,
                        format!(" {}", option),
                    );

                } else {
                    renderer.panel.print_ex(
                        panel_width / 2,
                        i as i32 + 1,
                        BackgroundFlag::None,
                        TextAlignment::Left,
                        format!("  {}", option),
                    );
                }
            }

        }

    } else if let Some(tile) = game_state.map
        .iter()
        .flatten()
        .find(|tile| tile.selected) {

        renderer.panel.print_ex(
            1,
            1,
            BackgroundFlag::None,
            TextAlignment::Left,
            format!("{}", tile.terrain)
        );

    }

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
