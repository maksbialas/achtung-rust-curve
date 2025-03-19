use crate::NUM_PLAYERS;

#[derive(Debug, Clone)]
pub enum Player {
    Fred,
    Greenlee,
    Pinkney,
    Bluebell,
    Willem,
    Greydon,
}

impl Player {
    pub fn all_players() -> [Self; NUM_PLAYERS] {
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
