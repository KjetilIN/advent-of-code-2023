use std::cmp::Ordering;

use crate::card::{Card, CardMethods};

#[derive(Debug)]
pub struct CardList{
    cards: Vec<Card>
}

/// Traits for the card list
pub trait CardListMethods {
    fn new(list: Vec<Card>) -> Self;
    fn sort_by_rank_and_cards(&mut self);
    fn sort_by_rank_and_cards_with_wildcard(&mut self);
    fn get_bid_score(&mut self)->u32;
    fn get_bid_score_with_wildcard(&mut self)->u32;

}

/// Implementation of the card list 
impl CardListMethods for CardList {

    fn new(list: Vec<Card>) -> Self{
        Self { cards: list }
    }

    fn sort_by_rank_and_cards(&mut self) {
        self.cards.sort_by(|c1, c2| {
            match c1.hand.score().cmp(&c2.hand.score()) {
                Ordering::Equal =>  return c1.compare_card(&c2),
                v => v,
            }
        });
    }

    fn sort_by_rank_and_cards_with_wildcard(&mut self) {
        self.cards.sort_by(|c1, c2| {
            match c1.hand.score().cmp(&c2.hand.score()) {
                Ordering::Equal =>  return c1.compare_card_with_wildcard(&c2),
                v => v,
            }
        });
    }

    fn get_bid_score(&mut self)->u32{
        self.sort_by_rank_and_cards();

        let mut rank = 1; 
        let mut sum = 0; 
        for card in self.cards.iter(){
            let card_sum = rank * card.bid;
            sum += card_sum;

            //println!("{} | {:?} | rank({})*bid({}) = {} | Total = {}", card.cards, card.hand, rank, card.bid, card_sum, sum);
            rank += 1;
        }

        sum
    }

    fn get_bid_score_with_wildcard(&mut self)->u32 {
        self.sort_by_rank_and_cards_with_wildcard();

        let mut rank = 1; 
        let mut sum = 0; 
        for card in self.cards.iter(){
            let card_sum = rank * card.bid;
            sum += card_sum;

            println!("{} | {:?} | rank({})*bid({}) = {} | Total = {}\n\n", card.cards, card.hand, rank, card.bid, card_sum, sum);
            rank += 1;
        }

        sum
    }
    
}

