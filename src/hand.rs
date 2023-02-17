use crate::card::Card;

pub struct Hand {
    hand: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {hand: vec![]}
    }
}

impl Hand {
    pub fn push_card(&mut self, card: Card){
        self.hand.push(card);
    }

    pub fn eval_hand(self) -> u8 {
        let mut hand_total = 0;

        for card in self.hand {
            hand_total += card.get_value();
        }

        hand_total
    }
}