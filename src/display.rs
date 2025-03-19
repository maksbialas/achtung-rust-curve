use crate::{PlayerState, NUM_PLAYERS, SCREEN_H, SCREEN_W};
use std::fmt;

pub struct Display([[u8; SCREEN_H]; SCREEN_W]);

impl Display {
    pub fn empty() -> Self {
        Display([[0; SCREEN_H]; SCREEN_W])
    }

    pub fn display_players(&mut self, player_states: &[PlayerState; NUM_PLAYERS]) {
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
