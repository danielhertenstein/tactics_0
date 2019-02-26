pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub selected: bool,
    pub selected_menu: Menu,
    pub move_range: i32,
    pub attack_range: i32,
}

#[derive(Clone)]
pub struct Tile {
    pub terrain: String,
    pub selected: bool,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            terrain: String::from("Grass"),
            selected: false,
        }
    }
}

type Map = Vec<Vec<Tile>>;

#[derive(PartialEq)]
pub enum PlayerState {
    MovingCursor,
    UnitSelected,
    MovingActor,
    ActorAttacking,
}

pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

type Menu = Vec<MenuOption>;

#[derive(Clone)]
pub enum MenuOption {
    Move,
    Attack,
}

impl std::fmt::Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MenuOption::Move => write!(f, "Move"),
            MenuOption::Attack => write!(f, "Attack"),
        }
    }
}

pub struct GameState {
    pub actors: Vec<Actor>,
    pub map: Map,
    pub player_state: PlayerState,
    pub cursor: Cursor,
    pub current_menu: Option<Menu>,
    pub current_menu_option: Option<usize>,
}

pub fn initial_game_state(map_height: i32, map_width: i32) -> GameState {
    GameState {
        actors: vec![
            Actor {
                x: 0,
                y: 0,
                name: String::from("Percy"),
                selected: false,
                selected_menu: vec![MenuOption::Move, MenuOption::Attack],
                move_range: 4,
                attack_range: 1,
            },
            Actor {
                x: 0,
                y: 1,
                name: String::from("Bad Guy"),
                selected: false,
                selected_menu: vec![MenuOption::Move, MenuOption::Attack],
                move_range: 3,
                attack_range: 1,
            },
        ],
        map: vec![vec![Tile::new(); map_height as usize]; map_width as usize],
        player_state: PlayerState::MovingCursor,
        cursor: Cursor { x: 0, y: 0 },
        current_menu: None,
        current_menu_option: None,
    }
}
