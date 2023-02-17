// use std::io;
// use std::cmp::Ordering;
// use crate::card::Card;
use crate::deck::Deck;
use crate::hand::Hand;

mod card;
mod deck;
mod hand;

fn main() {

    /* Original testing code, will be deprecated and removed when game functionality is implemented. */
    // let mut deck: Deck = Default::default();
    // let mut player_hand: Hand = Default::default();

    // for i in 0..53 {
    //     let card = deck.draw_card();

    //     match card {
    //         Some(card) => {
    //             println!("You drew {}", card.construct_name_str());
    //             println!("It has a value of {}", card.get_value());
    //             player_hand.push_card(card);
    //         },
    //         None => println!("The deck is empty!"),
    //     };

    //     counter += 1;
    // }


    // println!("The player's hand is {}", player_hand.eval_hand());

    /* Game loop */
    loop {
        println!("Welcome to Blackjack!");
        let mut deck: Deck = Default::default();
        let mut player_hand: Hand = Default::default();
        let mut house: Hand = Default::default();

        /* Initial turn */
        for i in 0..4 {
            let card = deck.draw_card();
            if i % 2 == 1 {
                match card {
                    Some(card) => {
                        player_hand.push_card(card);
                    },
                    None => println!("The deck is empty!")
                }
            } else {
                match card {
                    Some(card) => {
                        house.push_card(card);
                    },
                    None => println!("The deck is empty!")
                }
            }
        }

        println!("Your hand: {} It has a value of: {}", player_hand.to_string(false), player_hand.eval_hand());
        println!("House hand: {}", house.to_string(true));
        break;
    }
}
