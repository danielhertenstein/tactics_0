use game_state::*;
use input_handler::*;
use player_control::*;
use renderer::*;

const SCREEN_WIDTH: i32 = 30;
const SCREEN_HEIGHT: i32 = 20;

fn main() {
    let mut renderer = initialize_rendering_engine(SCREEN_HEIGHT, SCREEN_WIDTH);
    let mut game_state = initial_game_state(SCREEN_HEIGHT, SCREEN_WIDTH);

    loop {
        let input_state = capture_input_state();

        player_control_system(input_state, &mut game_state);

        render_system(&mut renderer, &game_state);
    }
}
