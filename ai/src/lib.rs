extern crate rand;

use rand::{thread_rng, Rng};

use game_state::{GameState, map_contains_position};

pub fn ai_control_system(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        // Move ai controlled characters one random tile for now. If the tile is blocked or is off
        // the edge of the map, the character does not move, but their charge time is reduced as if
        // they had moved.
        let mut rng = thread_rng();
        let direction: u32 = rng.gen_range(0, 4);
        let mut dx = 0;
        let mut dy = 0;
        match direction {
            0 => {
                dy = -1;
            },
            1 => {
                dx = 1;
            },
            2 => {
                dy = 1;
            },
            3 => {
                dx = -1;
            },
            _ => unreachable!()
        }
        let new_pos = &game_state.positions[index] + (dx, dy);

        let tile_unoccupied = game_state.positions
            .iter()
            .find(|&position| position == &new_pos)
            .is_none();

        if map_contains_position(&game_state.map, &new_pos) && tile_unoccupied {
            &mut game_state.positions[index].move_to(&new_pos);
        }

        game_state.charge_times[index] = 50;
        game_state.active_actor_index = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
