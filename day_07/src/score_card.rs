/// Used to core a card type for part 1 
pub fn score_char(c: char) -> Option<u8> {
    match c {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'J' => Some(10),
        'T' => Some(9),
        '9' => Some(8),
        '8' => Some(7),
        '7' => Some(6),
        '6' => Some(5),
        '5' => Some(4),
        '4' => Some(3),
        '3' => Some(2),
        '2' => Some(1),
        _ => None,
    }
}

/// Used to score a card for part 2 - where jack is the lowest value
pub fn score_char_with_wildcard(c: char) -> Option<u8> {
    match c {
        'A' => Some(13),
        'K' => Some(12),
        'Q' => Some(11),
        'T' => Some(10),
        '9' => Some(9),
        '8' => Some(8),
        '7' => Some(7),
        '6' => Some(6),
        '5' => Some(5),
        '4' => Some(4),
        '3' => Some(3),
        '2' => Some(2),
        'J' => Some(1),
        _ => None,
    }
}