mod display;
mod player;
mod settings;

use display::Display;
use player::Player;
use rand::Rng;
use settings::{NUM_PLAYERS, SCREEN_H, SCREEN_W};

#[derive(Debug)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    fn random() -> Self {
        match rand::rng().random_range(0..4) {
            0 => MoveDirection::Up,
            1 => MoveDirection::Down,
            2 => MoveDirection::Left,
            3 => MoveDirection::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Position(usize, usize);

impl Position {
    fn random() -> Self {
        let mut rng = rand::rng();
        Self(rng.random_range(0..SCREEN_W), rng.random_range(0..SCREEN_H))
    }
}

#[derive(Debug)]
struct PlayerState {
    player: Player,
    direction: MoveDirection,
    position: Position,
    alive: bool,
}

impl PlayerState {
    fn new(player: Player) -> Self {
        Self {
            player,
            direction: MoveDirection::random(),
            position: Position::random(),
            alive: true,
        }
    }

    fn all_player_states() -> [Self; NUM_PLAYERS] {
        Player::all_players().map(|p| Self::new(p))
    }

    fn tick(&mut self) {
        // TODO handle overflow
        match self.direction {
            MoveDirection::Up => self.position.1 += 1,
            MoveDirection::Down => self.position.1 -= 1,
            MoveDirection::Left => self.position.0 -= 1,
            MoveDirection::Right => self.position.0 += 1,
        }
    }
}

#[derive(Debug)]
struct Achtung {
    display: Display,
    player_states: [PlayerState; NUM_PLAYERS],
}

impl Achtung {
    fn new() -> Self {
        let player_states = PlayerState::all_player_states();
        let mut display = Display::empty();
        display.display_players(&player_states);

        Self {
            display,
            player_states,
        }
    }

    fn tick(&mut self) {
        self.player_states.iter_mut().for_each(|s| s.tick());
        self.display.display_players(&self.player_states);
    }
}

fn main() {
    let mut achtung = Achtung::new();
    achtung.tick();
    achtung.tick();
    achtung.tick();
    println!("Achtung! {achtung:?}");
}
