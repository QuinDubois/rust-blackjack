use crate::card::Card;

pub struct Hand {
    pub hand: Vec<Card>,
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

    pub fn to_string(&self, is_house: bool) -> String {
        let mut hand_str = String::from("");
        if is_house {
            hand_str.push_str(&self.hand[0].to_string());
            hand_str
        } else {
            for card in &self.hand {
                hand_str.push_str(&card.to_string());
                hand_str.push_str(", ");
            }
            hand_str
        }
    }
}