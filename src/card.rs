use std::fmt;

pub const DISPLAY_EMPTY: &'static str = "{   }";
pub const DISPLAY_BLANK: &'static str = "     ";
pub const DISPLAY_CARD_BACK: &'static str = "{###}";

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Suit {
  Heart,
  Diamond,
  Club,
  Spade,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
  Red,
  Black,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Value {
  Ace,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Card {
  pub suit: Suit,
  pub value: Value,
}

impl Card {
  // Helper Methods
  pub fn is_different_color(&self, card: &Card) -> bool {
    self.suit.is_different_color(&card.suit)
  }

  pub fn is_king(&self) -> bool {
    self.value.rank() == Value::King.rank()
  }

  pub fn is_one_more(&self, card: &Card) -> bool {
    1 == (card.value.rank() - self.value.rank())
  }

  pub fn is_one_less(&self, card: &Card) -> bool {
    1 == (self.value.rank() - card.value.rank())
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}{}{}{}",
      self.suit.color().open_char(),
      self.value,
      self.suit.color().middle_char(),
      self.suit,
      self.suit.color().close_char())
  }
}

#[derive(Debug)]
pub struct CardState {
  pub card: &'static Card,
  is_visible: bool,
}

impl CardState {
  pub fn new(card: &'static Card, is_visible: bool) -> CardState {
    CardState{card: card, is_visible: is_visible}
  }

  pub fn is_visible(&self) -> bool {
    self.is_visible
  }

  pub fn make_visible(&mut self) {
    self.is_visible = true;
  }
}

impl fmt::Display for CardState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.is_visible {
      true => write!(f, "{}", self.card),
      false => write!(f, "{}", DISPLAY_CARD_BACK)
    }
  }
}

impl Color {
  pub fn open_char(&self) -> char {
    match self {
      Color::Red => '(',
      Color::Black => '['
    }
  }

  pub fn close_char(&self) -> char {
    match self {
      Color::Red => ')',
      Color::Black => ']',
    }
  }

  pub fn middle_char(&self) -> char {
    match self {
      Color::Red => ':',
      Color::Black => '.',
    }
  }
}

impl Suit {
  pub fn color(&self) -> Color {
    match self {
      Suit::Heart => Color::Red,
      Suit::Diamond => Color::Red,
      Suit::Club => Color::Black,
      Suit::Spade => Color::Black,
    }
  }

  pub fn is_different_color(&self, other: &Suit) -> bool {
    self.color() != other.color()
  }

  pub fn code(&self) -> char {
    match self {
      Suit::Heart   => 'H',
      Suit::Diamond => 'D',
      Suit::Club    => 'C',
      Suit::Spade   => 'S',
    }
  }
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.code())
  }
}

impl Value {
  pub fn code(&self) -> char {
    match self {
      Value::Ace   => 'A',
      Value::Two   => '2',
      Value::Three => '3',
      Value::Four  => '4',
      Value::Five  => '5',
      Value::Six   => '6',
      Value::Seven => '7',
      Value::Eight => '8',
      Value::Nine  => '9',
      Value::Ten   => 'T',
      Value::Jack  => 'J',
      Value::Queen => 'Q',
      Value::King  => 'K',
    }
  }

  pub fn rank(&self) -> i8 {
    match self {
      Value::Ace   => 1,
      Value::Two   => 2,
      Value::Three => 3,
      Value::Four  => 4,
      Value::Five  => 5,
      Value::Six   => 6,
      Value::Seven => 7,
      Value::Eight => 8,
      Value::Nine  => 9,
      Value::Ten   => 10,
      Value::Jack  => 11,
      Value::Queen => 12,
      Value::King  => 13,
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.code())
  }
}

#[cfg(test)]
mod tests {
  use crate::card::*;
  use hamcrest2::prelude::*;

  #[test]
  fn test_four_of_hearts() {
    let card = Card{suit: Suit::Heart, value: Value::Four};
    assert_that!(format!("{}", card), eq("(4:H)"));
  }

  #[test]
  fn test_jack_of_clubs() {
    let card = Card{suit: Suit::Club, value: Value::Jack};
    assert_that!(format!("{}", card), eq("[J.C]"));
  }

  #[test]
  fn test_ace_of_diamonds() {
    let card = Card{suit: Suit::Diamond, value: Value::Ace};
    assert_that!(format!("{}", card), eq("(A:D)"));
  }

  #[test]
  fn test_queen_of_spades() {
    let card = Card{suit: Suit::Spade, value: Value::Queen};
    assert_that!(format!("{}", card), eq("[Q.S]"));
  }

  #[test]
  fn test_suit_different_color_true() {
    let card1 = Card{suit: Suit::Spade, value: Value::Queen};
    let card2 = Card{suit: Suit::Diamond, value: Value::Nine};
    assert_eq!(card1.suit.is_different_color(&card2.suit), true);
  }

  #[test]
  fn test_suit_different_color_false() {
    let card1 = Card{suit: Suit::Spade, value: Value::Queen};
    let card2 = Card{suit: Suit::Club, value: Value::Nine};
    assert_eq!(card1.suit.is_different_color(&card2.suit), false);
  }

  #[test]
  fn test_rank_diff() {
    let card1 = Card{suit: Suit::Spade, value: Value::Queen};
    let card2 = Card{suit: Suit::Spade, value: Value::King};

    let diff = card1.value.rank() - card2.value.rank();
    assert_eq!(diff, -1);
  }

  #[test]
  fn test_create_card_state() {
    let card :&'static Card = &Card{suit: Suit::Spade, value: Value::Queen};
    let card_state = CardState::new(card, false);
    assert_that!(card_state.card, eq(card));
    assert_that!(card_state.is_visible, is(false));
  }

  #[test]
  fn test_card_state_make_visible() {
    let card :&'static Card = &Card{suit: Suit::Spade, value: Value::Queen};
    let mut card_state = CardState::new(card, false);
    card_state.make_visible();
    assert_that!(card_state.is_visible(), is(true));
  }

  #[test]
  fn test_card_same_color() {
    let queen_of_spades = Card{suit: Suit::Spade, value: Value::Queen};
    let nine_of_clubs :&'static Card = &Card{suit: Suit::Club, value: Value::Nine};
    let two_of_hearts :&'static Card = &Card{suit: Suit::Heart, value: Value::Two};

    assert_that!(queen_of_spades.is_different_color(nine_of_clubs), is(false));
    assert_that!(queen_of_spades.is_different_color(two_of_hearts), is(true));
  }

  #[test]
  fn test_card_one_more() {
    let jack_of_hearts = Card{suit: Suit::Heart, value: Value::Jack};
    let king_of_clubs :&'static Card = &Card{suit: Suit::Club, value: Value::King};
    let jack_of_spades :&'static Card = &Card{suit: Suit::Spade, value: Value::Jack};
    let queen_of_hearts :&'static Card = &Card{suit: Suit::Heart, value: Value::Queen};
    let ten_of_diamonds :&'static Card = &Card{suit: Suit::Diamond, value: Value::Ten};

    assert_that!(jack_of_hearts.is_one_more(queen_of_hearts), is(true));

    assert_that!(jack_of_hearts.is_one_more(king_of_clubs), is(false));
    assert_that!(jack_of_hearts.is_one_more(jack_of_spades), is(false));    
    assert_that!(jack_of_hearts.is_one_more(ten_of_diamonds), is(false));
  }

  #[test]
  fn test_card_one_less() {
    let jack_of_hearts = Card{suit: Suit::Heart, value: Value::Jack};
    let king_of_clubs :&'static Card = &Card{suit: Suit::Club, value: Value::King};
    let jack_of_spades :&'static Card = &Card{suit: Suit::Spade, value: Value::Jack};
    let queen_of_hearts :&'static Card = &Card{suit: Suit::Heart, value: Value::Queen};
    let ten_of_diamonds :&'static Card = &Card{suit: Suit::Diamond, value: Value::Ten};

    assert_that!(jack_of_hearts.is_one_less(ten_of_diamonds), is(true));

    assert_that!(jack_of_hearts.is_one_less(king_of_clubs), is(false));
    assert_that!(jack_of_hearts.is_one_less(jack_of_spades), is(false));    
    assert_that!(jack_of_hearts.is_one_less(queen_of_hearts), is(false));
  }
}