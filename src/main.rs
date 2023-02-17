use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;

mod card;
mod deck;
mod hand;

fn main() {
    let mut deck: Deck = Default::default();
    let mut player_hand: Hand = Default::default();

    let mut counter = 0;

    while counter < 2 {
        let card = deck.draw_card();

        match card {
            Some(card) => {
                println!("You drew {}", card.construct_name_str());
                println!("It has a value of {}", card.get_value());
                player_hand.push_card(card);
            },
            None => println!("The deck is empty!"),
        };

        counter += 1;
    }

    println!("The player's hand is {}", player_hand.eval_hand());
}
