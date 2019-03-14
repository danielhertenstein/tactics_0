extern crate tcod;

use tcod::chars;
use tcod::colors;
use tcod::console::*;

use game_state::{GameState, PlayerState};

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
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
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
    render_map(renderer, game_state);
    render_panel(renderer, game_state);

    renderer.root.flush();

    clear_actors(renderer, game_state);
}

fn render_map(renderer: &mut Renderer, game_state: &GameState) {
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

    for (actor, position) in game_state.actors
        .iter()
        .zip(game_state.positions.iter()) {
        let color = match actor.player_controlled {
            true => colors::BLUE,
            false => colors::RED,
        };
        renderer.map.set_default_foreground(color);
        renderer.map.put_char(
            position.x,
            position.y,
            'A',
            BackgroundFlag::None
        );
    }

    match &game_state.player_state {
        PlayerState::MovingActor => {
            let active_index = game_state.active_actor_index.unwrap();
            let actor = &game_state.actors[active_index];
            let actor_position = &game_state.positions[active_index];

            let color = match actor.player_controlled {
                true => colors::LIGHT_BLUE,
                false => colors::LIGHT_RED,
            };

            for x in -actor.move_range..=actor.move_range {
                for y in -actor.move_range..=actor.move_range {
                    let new_x = actor_position.x + x;
                    let new_y = actor_position.y + y;

                    let other_actor_under_cursor = game_state.positions
                        .iter()
                        .position(|position| position.x == new_x && position.y == new_y);

                    match other_actor_under_cursor {
                        Some(index) if index != active_index => continue,
                        _ => {}
                    }

                    if x.abs() + y.abs() > actor.move_range {
                        continue
                    }

                    if new_x > map_width || new_x < 0 || new_y > map_width || new_y < 0 {
                        continue
                    }

                    renderer.map.set_char_background(
                        new_x,
                        new_y,
                        color,
                        BackgroundFlag::Set
                    );
                }
            }
        },
        PlayerState::ActorAttacking => {
            let active_index = game_state.active_actor_index.unwrap();
            let actor = &game_state.actors[active_index];
            let actor_position = &game_state.positions[active_index];

            for x in -actor.attack_range..=actor.attack_range {
                for y in -actor.attack_range..=actor.attack_range {
                    if x.abs() + y.abs() > actor.attack_range || (x == 0 && y == 0)  {
                        continue
                    }

                    let new_x = actor_position.x + x;
                    let new_y = actor_position.y + y;

                    if new_x > map_width || new_x < 0 || new_y > map_width || new_y < 0 {
                        continue
                    }

                    renderer.map.set_char_background(
                        new_x,
                        new_y,
                        colors::BRASS,
                        BackgroundFlag::Set
                    );
                }
            }
        }
        _ => {},
    }

    renderer.map.set_char_background(
        game_state.cursor.x,
        game_state.cursor.y,
        colors::LIGHT_GREY,
        BackgroundFlag::Set,
    );

    blit(
        &renderer.map,
        (0, 0),
        (map_width, map_height),
        &mut renderer.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn render_panel(renderer: &mut Renderer, game_state: &GameState) {
    renderer.panel.set_default_background(colors::BLACK);
    renderer.panel.clear();

    let panel_width = renderer.panel.width();
    let panel_height = renderer.panel.height();

    match &game_state.player_state {
        PlayerState::MovingCursor => {
            match game_state.positions
                .iter()
                .position(|position| position == &game_state.cursor) {
                Some(index) => show_actor_info(renderer, game_state, index),
                None => {
                    let cursor_x = game_state.cursor.x;
                    let cursor_y = game_state.cursor.y;
                    let tile =  &game_state.map[cursor_x as usize][cursor_y as usize];
                    renderer.panel.print_ex(
                        1,
                        1,
                        BackgroundFlag::None,
                        TextAlignment::Left,
                        format!("{}", tile.terrain)
                    );
                }
            }
        },
        PlayerState::UnitSelected => {
            match game_state.positions
                .iter()
                .position(|position| position == &game_state.cursor) {
                Some(index) => show_actor_info(renderer, game_state, index),
                _ => unreachable!()
            }

            if let Some(menu) = &game_state.menu {
                for (i, option) in menu.options.iter().enumerate() {
                    if menu.selected_index == i {
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
        },
        PlayerState::MovingActor => {
            let actor = &game_state.actors[game_state.active_actor_index.unwrap()];

            renderer.panel.print_ex(
                1,
                1,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("{}", actor.name)
            );

            renderer.panel.print_ex(
                1,
                2,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("Select a tile to move to"),
            );
        },
        PlayerState::ActorAttacking => {
            let actor = &game_state.actors[game_state.active_actor_index.unwrap()];

            renderer.panel.print_ex(
                1,
                1,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("{}", actor.name)
            );

            renderer.panel.print_ex(
                1,
                2,
                BackgroundFlag::None,
                TextAlignment::Left,
                format!("Select a tile to attack"),
            );
        }
        _ => {}
    }

    let map_height = renderer.map.height();
    blit(
        &renderer.panel,
        (0, 0),
        (panel_width, panel_height),
        &mut renderer.root,
        (0, map_height),
        1.0,
        1.0,
    );
}

fn show_actor_info(renderer: &mut Renderer, game_state: &GameState, index: usize) {
    renderer.panel.print_ex(
        1,
        1,
        BackgroundFlag::None,
        TextAlignment::Left,
        format!("{}", game_state.actors[index].name),
    );
    renderer.panel.print_ex(
        1,
        2,
        BackgroundFlag::None,
        TextAlignment::Left,
        format!("CT: {}/100", game_state.charge_times[index]),
    );
}

fn clear_actors(renderer: &mut Renderer, game_state: &GameState) {
    for actor_position in &game_state.positions {
        renderer.map.put_char(
            actor_position.x,
            actor_position.y,
            ' ',
            BackgroundFlag::None);
    }
}