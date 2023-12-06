# Day 4 - Description

Given a list of scratch card:
```text
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

Where the left side is winning numbers, and the right side is the card numbers

1. Find the total amount of points of all cards
    a. A match is given 1 point and then every match after that the point is doubled
2. Given that each card match for card id `n`, adds `n+1, n+2, .. n+matches` card clones - find out how many cards you process in total

Link to complete description: https://adventofcode.com/2023/day/4

## Approach/Algorithm

To solve this challenge, I created a Card struct:

```rust 
#[derive(Clone)]
struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
    matches: u32,
}
```

To find the point for a single card
1. Iterate over each winner number
2. Check if the winning number is in the card number vector
3. If it is add the total points based on if it is a match or not


For the second part, I created a Card Container struct:

```rust 
struct CardContainer {
    cards_processed: u32,
    card_map: HashMap<u32, Card>,
    cards: Vec<Card>,
}
```

The card map is a to access cards, given the ID as key. It is never modified.
With this struct, I created a function for processing card numbers - `process_cards`:

1. Create a new `cards_to_process` vector. (Could have modified the original card vector, but decided to clone it)
2. While we have cards to processed
    a. Remove the first card from `cards_to_process`.
    b. Find the amount of matches for that card (the number of matches is created by the `Card` constructor)
    c. Increment the amount of cards processed by 1
    d. Add new card copies to `cards_to_process` (based on how many matches the card has)


## Code Snippets

The processing function: 

```rust 
// Process all the cards and
// sets the amount of cards processed to 0
// Will remove all cards from the list
fn process_cards(&mut self) {
    // Set the amount of cards processed to 0  
    self.cards_processed = 0;

    // Clone the cards that is available 
    let mut cards_to_process: Vec<Card> = self.cards.clone();


    // While we still have cards to process
    while !cards_to_process.is_empty() {
        // Remove the card from the vector 
        let current_card: Card = cards_to_process.swap_remove(0);

        // Increment the card processed count
        self.cards_processed += 1;

        // Add copies to be processed
        // The cards that now needs to be processed, given card id n:
        // - Add card n+1, n+2, ... , n+matches
        // Each card need to be processed 
        let mut new_card_index = current_card.card_id + 1;

        // While there is cards to add
        // Will jump over this section when there if there was no matches 
        while new_card_index <= (current_card.card_id + current_card.matches) {

            // Get the card from the card-map and clone it 
            let card_to_add: Card = match self.card_map.get(&new_card_index) {
                Some(card) => card.clone(),
                None => {
                    eprintln!("Error cloning card nr {}", new_card_index);
                    std::process::exit(1);
                }
            };
            
            // Add the cards to be processes
            cards_to_process.push(card_to_add);

            new_card_index += 1;
        }
    }
}
```


