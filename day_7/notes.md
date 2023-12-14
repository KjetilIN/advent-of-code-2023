# Day 7 - Description

> Note: Did not complete part 2 

Given a list of cards, classify each card. Then rank the card and sum up the rank multiplied with the bid of the card

Link to complete description: https://adventofcode.com/2023/day/7

## Approach/Algorithm

I created a card struct that represent the card: 

```rust
#[derive(Debug, Clone)]
pub struct Card {
    pub hand: Hand,
    pub cards: String,
    pub bid: u32,
}

/// Trait for the card
pub trait CardMethods {
    fn new(line: &str, has_wild_card: bool) -> Option<Self> where Self: Sized;
    fn compare_card(&self, compared_card: &Card) -> Ordering;
    fn compare_card_with_wildcard(&self, compared_card: &Card) -> Ordering;
}
```

Then I created a Card list struct for handling a list of cards:
```rust
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
```

For solving part 1:
1. Classify card when we create a new card with `classify_card()` method
2. Create a new `CardList` instance 
3. Sort each card with the `compare_card()` method
    1. Sort by `Hand` type first
    2. Sort then by value one by one until one is lower than the other
4. After we have sorted each card, we iterate through each card and find the sum card with rank multiplied with bid

## Learnings

This was a hard challenge.

1. Creating tests are useful
2. Edge cases are hard to find
3. Splitting the code into multiple files is smart 

## Code Snippets

I found the comparing method very useful:

```rust
    fn sort_by_rank_and_cards(&mut self) {
            self.cards.sort_by(|c1, c2| {
                match c1.hand.score().cmp(&c2.hand.score()) {
                    Ordering::Equal =>  return c1.compare_card(&c2),
                    v => v,
                }
            });
        }
```

This method uses the compare cards method:

```rust
    /// Method for comparing two cards, part 1 
    fn compare_card(&self, compared_card: &Card) -> Ordering{
        for (index, char) in self.cards.chars().enumerate(){
            let current_char_score: u8 =  match score_char(char){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", char, self.cards);
                    exit(1)
                }
            };


            let compared_card_score: u8 = match score_char(compared_card.cards.chars().nth(index).unwrap()){
                Some(order) => order,
                None =>{
                    eprintln!("ERROR: retrieving the char order for char '{}' in cards: {}", compared_card.cards.chars().nth(index).unwrap(), compared_card.cards);
                    exit(1)
                }
            };

            match current_char_score.cmp(&compared_card_score) {
                Ordering::Equal => continue,
                non_equal_ordering => return non_equal_ordering,
            }
        }

        // Compare entire card strings when scores are equal
        self.cards.cmp(&compared_card.cards)
    }
```

