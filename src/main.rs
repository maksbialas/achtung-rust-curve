use rand::Rng;
use std::fmt;

const NUM_PLAYERS: usize = 6;
const SCREEN_H: usize = 64;
const SCREEN_W: usize = 64;

struct Display([[u8; SCREEN_H]; SCREEN_W]);

impl Display {
    fn empty() -> Self {
        Display([[0; SCREEN_H]; SCREEN_W])
    }

    fn display_players(&mut self, player_states: &[PlayerState; NUM_PLAYERS]) {
        for state in player_states {
            self.0[state.position.0][state.position.1] = state.player.clone().into();
        }
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n┌{}┐", "-".repeat(SCREEN_W))?;
        for row in &self.0 {
            write!(f, "|")?;
            for &cell in row {
                let str = if cell == 0 {
                    " ".to_string() // Return a space if the value is 0
                } else {
                    cell.to_string() // Otherwise, convert the u8 to a string
                };
                write!(f, "{str}")?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "└{}┘", "-".repeat(SCREEN_W))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Player {
    Fred,
    Greenlee,
    Pinkney,
    Bluebell,
    Willem,
    Greydon,
}

impl Player {
    fn all_players() -> [Self; NUM_PLAYERS] {
        [
            Self::Fred,
            Self::Greenlee,
            Self::Pinkney,
            Self::Bluebell,
            Self::Willem,
            Self::Greydon,
        ]
    }
}

impl Into<u8> for Player {
    fn into(self) -> u8 {
        match self {
            Self::Fred => 1,
            Self::Greenlee => 2,
            Self::Pinkney => 3,
            Self::Bluebell => 4,
            Self::Willem => 5,
            Self::Greydon => 6,
        }
    }
}

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
}

impl PlayerState {
    fn new(player: Player) -> Self {
        Self {
            player,
            direction: MoveDirection::random(),
            position: Position::random(),
        }
    }

    fn all_player_states() -> [Self; NUM_PLAYERS] {
        Player::all_players().map(|p| Self::new(p))
    }

    fn tick(&mut self) {
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
        for state in &mut self.player_states {
            state.tick();
        }
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
