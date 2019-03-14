use game_state::{GameState, PlayerState};

pub fn clock_tick_system(game_state: &mut GameState) {
    // Look for charge times greater than or equal to 100. If there are multiple, tie-break by who
    // has the highest charge time. If there is a tie there, tie break by speed. If there is a tie
    // there, tie break by lowest index.
    let turn_ready_for = game_state.charge_times
        .iter()
        .zip(game_state.combat_stats.iter().map(|cs| &cs.speed))
        .enumerate()
        .filter(|&(_i, (&c, _s))| c >= 100)
        .max_by(|&(_i1, (&c1, &s1)), &(_i2, (&c2, &s2))| {
            if c1 == c2 {
                s1.cmp(&s2)
            } else {
                c1.cmp(&c2)
            }
        })
        .map(|(i, (_c, _s))| i);

    match turn_ready_for {
        Some(index) => {
            game_state.active_actor_index = Some(index);
            game_state.charge_times[index] = 100;
            if game_state.actors[index].player_controlled {
                game_state.cursor.move_to(&game_state.positions[index]);
                game_state.player_state = PlayerState::TurnReady;
            }
        },
        None => {
            for i in 0..game_state.charge_times.len() {
                game_state.charge_times[i] += game_state.combat_stats[i].speed;
            }

        },
    }
}

