pub struct Actor {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Tile;

impl Tile {
    pub fn new() -> Tile {
        Tile {}
    }
}

type Map = Vec<Vec<Tile>>;

pub enum PlayerState {
    MovingCursor,
    UnitSelected,
}

pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

pub struct GameState {
    pub actors: Vec<Actor>,
    pub map: Map,
    pub player_state: PlayerState,
    pub cursor: Cursor,
}

pub fn initial_game_state(screen_height: i32, screen_width: i32) -> GameState {

    GameState {
        actors: vec![
            Actor { x: 0, y: 0 },
            Actor { x: 0, y: 1 },
        ],
        map: vec![vec![Tile::new(); screen_height as usize]; screen_width as usize],
        player_state: PlayerState::MovingCursor,
        cursor: Cursor { x: 0, y: 0 }
    }
}
