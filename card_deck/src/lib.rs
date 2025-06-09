use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Heart, 
    Diamond, 
    Spade,
    Club,
   
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    Ace, 
    King, 
    Queen,
    Jack,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        let  rng = rand::thread_rng().gen_range(1..=4) as u8;
        match rng {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!("What The fuck")
        }
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => panic!("What The fuck")
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let  rng = rand::thread_rng().gen_range(1..=4) as u8;
        match rng {
            1 => Self::Ace,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            rng @ 2..11 => Self::Number(rng),
            _ => panic!("What The fuck")
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            value @ 2..11 => Self::Number(value),
            _ => panic!("What The fuck")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}