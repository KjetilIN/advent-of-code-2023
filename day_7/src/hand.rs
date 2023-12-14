#[derive(Debug, Clone)]
pub enum Hand {
    FullHouse,
    FiveOfKind,
    FourOfKind,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn score(&self)-> u8{
        match self{
            Hand::FiveOfKind => 7,
            Hand::FourOfKind => 6,
            Hand::FullHouse => 5,
            Hand::ThreeOfKind => 4,
            Hand::TwoPair => 3,
            Hand::OnePair => 2,
            Hand::HighCard => 1,
        }
    } 
    
}