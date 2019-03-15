use combat::*;
use game_state::{GameState, map_contains_position, Turn};

pub fn ai_control_system(game_state: &mut GameState) {
    if let Some(index) = game_state.active_actor_index {
        game_state.turn = Some(Turn::new());

        let ai_pos = game_state.positions[index].clone();

        // Find closest player controlled unit
        let target_index = game_state.positions
            .iter()
            .zip(game_state.actors.iter())
            .enumerate()
            .filter(|(_idx, (_pos, actor))| actor.player_controlled)
            .map(|(idx, (pos, _actor))| {
                (idx,  ai_pos.distance_to(pos))
            })
            .min_by(|(_idx1, dist1), (_idx2, dist2)| dist1.cmp(dist2))
            .map(|(idx, _dist)| idx)
            .expect("If there are no player controlled units for the AI to attack, \
            you should have lost.");
        let target_pos = game_state.positions[target_index].clone();

        // Move towards them
        // For each tile the unit could move to, which tile minimizes the distance to the target.
        // If we find a tile within the attack range, stop and move there.
        let attack_range = game_state.combat_stats[index].attack_range;
        let move_range = game_state.combat_stats[index].move_range;

        // If too far away to attack, move first
        if ai_pos.distance_to(&target_pos) > attack_range {
            'outer: for move_x in -move_range..=move_range {
                'inner: for move_y in -move_range..=move_range {
                    let new_pos = &ai_pos + (move_x, move_y);
                    if !map_contains_position(&game_state.map, &new_pos) {
                        continue 'inner;
                    }
                    // If there is someone else on that tile, we can't move there
                    if game_state.positions
                        .iter()
                        .find(|&position| position == &new_pos && position != &ai_pos)
                        .is_some() {
                        continue 'inner;
                    }
                    // If the tile puts us in attack range, stop and move there
                    if new_pos.distance_to(&target_pos) <= attack_range {
                        game_state.positions[index].move_to(&new_pos);
                        match game_state.turn.as_mut() {
                            Some(turn) => turn.moved = true,
                            None => {},
                        }
                        break 'outer;
                    }
                }
            }
        }

        // Attack if able
        if game_state.positions[index].distance_to(&target_pos) <= attack_range {
            attack(&mut game_state.combat_stats, index, target_index);

            match game_state.turn.as_mut() {
                Some(turn) => turn.acted = true,
                None => {},
            }

            let dead_indices = check_if_anyone_died(&game_state.combat_stats);
            for dead_index in dead_indices {
                game_state.remove_entity(dead_index);
            }
        }


        let mut new_charge_time = 80;
        match &game_state.turn {
            Some(turn) => {
                if turn.moved {
                    new_charge_time -= 30;
                }
                if turn.acted {
                    new_charge_time -= 50;
                }
            },
            None => {}
        }
        game_state.charge_times[index] = new_charge_time;
        game_state.active_actor_index = None;
        game_state.turn = None;
        let name = &game_state.actors[index].name;
        println!("{} took its turn!", name);
    }
}
