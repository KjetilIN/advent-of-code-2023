#[cfg(test)]
mod tests {

    use std::vec;
    use crate::{
        card::{Card, CardMethods},
        card_list::{CardList, CardListMethods},
        classify_card::{classify_card_with_wildcard, count_jokers},
    };

    #[test]
    fn test_joker_count() {
        let test_card_set = "KTJJT 220";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let joker_count = count_jokers(&card.cards);
        assert_eq!(joker_count, 2)
    }

    #[test]
    fn test_classify_wild_five_of_kind() {
        let test_card_set = "JQJJQ 94832";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::FiveOfKind);
    }

    #[test]
    fn test_classify_wild_four_of_kind_valid_1() {
        let test_card_set = "KTJJT 220";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::FourOfKind);
    }

    #[test]
    fn test_classify_wild_four_of_kind_valid_2() {
        let test_card_set = "QQQJA 483";
        let card_option: Option<Card> = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::FourOfKind);
    }

    #[test]
    fn test_classify_wild_three_of_kind_1() {
        let test_card_set = "QKJ8K 32";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::ThreeOfKind);
    }

    #[test]
    fn test_classify_wild_three_of_kind_2() {
        let test_card_set = "3ATTJ 245";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::ThreeOfKind);
    }

    #[test]
    fn test_classify_wild_three_of_kind_3() {
        let test_card_set = "JJQ34 245";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::ThreeOfKind);
    }

    #[test]
    fn test_classify_wild_three_of_kind_4() {
        let test_card_set = "4T48J 245";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::ThreeOfKind);
    }

    #[test]
    fn test_classify_wild_three_of_kind_5() {
        let test_card_set = "234JJ 245";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::ThreeOfKind);
    }

    #[test]
    fn test_classify_wild_full_house_1() {
        let test_card_set = "2233J 245";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::FullHouse);
    }

    #[test]
    fn test_classify_wild_pair_1() {
        let test_card_set = "29TJQ 605";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::OnePair);
    }

    #[test]
    fn test_classify_wild_pair_2() {
        let test_card_set = "29TJQ 605";
        let card_option = Card::new(test_card_set, true);

        // Check if the Card::new function succeeded
        assert!(card_option.is_some(), "Failed to create card for the test");

        // If successful, unwrap the result to get the card object
        let card = card_option.unwrap();

        let hand = classify_card_with_wildcard(&card.cards);

        assert!(hand.is_ok(), "Failed to classify the card");

        let hand = hand.unwrap();
        assert_eq!(hand, crate::hand::Hand::OnePair);
    }

    #[test]
    fn test_calculate_bid_with_wild_1() {
        let test_cards = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let mut card_vec: Vec<Card> = Vec::new();

        for card in test_cards {
            let card_option = Card::new(card, true);

            // Check if the Card::new function succeeded
            assert!(card_option.is_some(), "Failed to create card for the test");

            // If successful, unwrap the result to get the card object
            let card = card_option.unwrap();
            card_vec.push(card);
        }

        let mut card_list = CardList::new(card_vec);

        assert_eq!(card_list.get_bid_score_with_wildcard(), 5905);
    }

    #[test]
    fn test_calculate_bid_with_wild_2() {
        let test_cards = vec![
            "2345A 1", "Q2KJJ 13", "Q2Q2Q 19", "T3T3J 17", "T3Q33 11", "2345J 3", "J345A 2",
            "32T3K 5", "T55J5 29", "KK677 7", "KTJJT 34", "QQQJA 31", "JJJJJ 37", "JAAAA 43",
            "AAAAJ 59", "AAAAA 61", "2AAAA 23", "2JJJJ 53", "JJJJ2 41",
        ];
        let mut card_vec: Vec<Card> = Vec::new();

        for card in test_cards {
            let card_option = Card::new(card, true);

            // Check if the Card::new function succeeded
            assert!(card_option.is_some(), "Failed to create card for the test");

            // If successful, unwrap the result to get the card object
            let card = card_option.unwrap();
            card_vec.push(card);
        }

        let mut card_list = CardList::new(card_vec);

        assert_eq!(card_list.get_bid_score_with_wildcard(), 6839);
    }

    #[test]
    fn test_calculate_bid_with_wild_3() {
        let test_cards = vec![
            "2345A 1",
            "Q2KJJ 13",
            "Q2Q2Q 19",
            "T3T3J 17",
            "T3Q33 11",
            "2345J 3",
            "J345A 2",
            "32T3K 5",
            "T55J5 29",
            "KK677 7",
            "KTJJT 34",
            "QQQJA 31",
            "JJJJJ 37",
            "JAAAA 43",
            "AAAAJ 59",
            "AAAAA 61",
            "2AAAA 23",
            "2JJJJ 53",
            "JKQKK 21",
            "JJJJ2 41 "
        ];
        let mut card_vec: Vec<Card> = Vec::new();

        for card in test_cards {
            let card_option = Card::new(card, true);

            // Check if the Card::new function succeeded
            assert!(card_option.is_some(), "Failed to create card for the test");

            // If successful, unwrap the result to get the card object
            let card = card_option.unwrap();
            card_vec.push(card);
        }

        let mut card_list = CardList::new(card_vec);

        assert_eq!(card_list.get_bid_score_with_wildcard(), 7460);
    }
}
