use std::fmt;

use crate::card;

pub struct Foundation {
  cards: Vec<&'static card::Card>,
  suit: card::Suit,
}

impl Foundation {
  pub fn new(suit: card::Suit) -> Foundation {
    Foundation{cards: Vec::new(), suit}
  }

  pub fn get_top(&self) -> Option<&'static card::Card> {
    if self.cards.is_empty() {
      None
    }
    else {
      Some(self.cards[self.cards.len() - 1])
    }
  }

  pub fn take(&mut self) -> Option<&'static card::Card> {
    self.cards.pop()
  }

  pub fn is_full(&self) -> bool {
    match self.get_top() {
      Some(top_card) => top_card.value == card::Value::King,
      None => false,
    }
  }

  pub fn can_add(&self, card: &'static card::Card) -> bool {
    match self.get_top() {
      Some(top_card) => card.suit == self.suit && top_card.is_one_more(card),
      None => card.suit == self.suit && card.value == card::Value::Ace,
    }
  }

  pub fn add(&mut self, card: &'static card::Card) {
    self.cards.push(card);
  }
}

impl fmt::Display for Foundation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.get_top() {
      Some(card) => write!(f, "{}", card),
      None => write!(f, "{}", card::DISPLAY_EMPTY)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::card;
  use crate::deck;
  use crate::foundation::Foundation;
  use crate::deck::FULL_DECK;
  use hamcrest2::prelude::*;

  #[test]
  fn test_create_foundation() {
    let fdh = Foundation::new(card::Suit::Heart);
    assert_that!(fdh.is_full(), is(false));
    let top = fdh.get_top();
    assert_that!(top.is_none(), is(true));
  }

  #[test]
  fn test_can_add_empty() {
    let fdh = Foundation::new(card::Suit::Heart);
    assert_that!(fdh.can_add(deck::ACE_OF_HEARTS),   is(true));
    assert_that!(fdh.can_add(deck::ACE_OF_SPADES),   is(false));
    assert_that!(fdh.can_add(deck::ACE_OF_CLUBS),    is(false));
    assert_that!(fdh.can_add(deck::ACE_OF_DIAMONDS), is(false));
    assert_that!(fdh.can_add(deck::TWO_OF_HEARTS),   is(false));
  }

  #[test]
  fn test_can_add() {
    let mut fdh = Foundation::new(card::Suit::Heart);
    fdh.add(deck::ACE_OF_HEARTS);
    assert_that!(fdh.can_add(deck::TWO_OF_HEARTS), is(true));
    fdh.add(deck::TWO_OF_HEARTS);

    assert_that!(fdh.cards.len(), eq(2));
    assert_that!(fdh.can_add(deck::FOUR_OF_HEARTS), is(false));
  }

  #[test]
  fn test_is_full_true() {
    let mut fdh = Foundation::new(card::Suit::Heart);
    for card in FULL_DECK.iter().take(13) {
      fdh.add(card);
    }
    assert_that!(fdh.is_full(), is(true));
  }

  #[test]
  fn test_is_full_false() {
    let mut fdh = Foundation::new(card::Suit::Heart);
    fdh.add(deck::ACE_OF_HEARTS);
    fdh.add(deck::TWO_OF_HEARTS);

    assert_that!(fdh.is_full(), is(false));
  }

  #[test]
  fn test_display_empty() {
    let fdh = Foundation::new(card::Suit::Heart);
    assert_that!(format!("{}", fdh), eq("{   }"));
  }

  #[test]
  fn test_display() {
    let mut fdh = Foundation::new(card::Suit::Heart);
    fdh.add(deck::ACE_OF_HEARTS);
    fdh.add(deck::TWO_OF_HEARTS);
    assert_that!(format!("{}", fdh), eq("(2:H)"));
  }
}
