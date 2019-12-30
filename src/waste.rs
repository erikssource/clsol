use std::fmt;

use crate::card;

pub struct Waste {
  cards: Vec<&'static card::Card>
}

impl Waste {
  pub fn new() -> Waste {
    Waste{cards: Vec::new()}
  }

  pub fn size(&self) -> usize {
    self.cards.len()
  }

  pub fn put(&mut self, take: Vec<&'static card::Card>) {
    for card in take {
      self.cards.push(card);
    }
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

  pub fn get_all(&self) -> &Vec<&'static card::Card> {
    &self.cards
  }

  pub fn clear(&mut self) {
    self.cards.clear();
  }
}

impl fmt::Display for Waste {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.get_top() {
      Some(card) => write!(f, "{}", card),
      None => write!(f, "{}", card::DISPLAY_EMPTY)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::waste;
  use crate::deck;
  use crate::card;
  use hamcrest2::prelude::*;

  fn check_taken(opt: Option<&'static card::Card>, card: &'static card::Card) {
    match opt {
      Some(taken) => assert_that!(taken, eq(card)),
      None => panic!(),
    }
  }

  #[test]
  fn test_create_waste() {
    let waste = waste::Waste::new();
    assert_that!(waste.cards.is_empty(), is(true));
  }

  #[test]
  fn test_display_empty() {
    let waste = waste::Waste::new();
    assert_that!(format!("{}", waste), eq("{   }"));
  }

  #[test]
  fn test_display() {
    let mut waste = waste::Waste::new();
    let take = vec!(deck::ACE_OF_HEARTS);
    waste.put(take);
    assert_that!(format!("{}", waste), eq("(A:H)"));
  }

  #[test]
  fn test_put() {
    // three card take
    let take = vec!(deck::ACE_OF_HEARTS, deck::TWO_OF_HEARTS, deck::THREE_OF_HEARTS);
    let mut waste = waste::Waste::new();
    waste.put(take);
    // top should be top of take (3H)
    let opt = waste.get_top();
    match opt {
      Some(top_card) => assert_that!(top_card, eq(deck::THREE_OF_HEARTS)),
      None => panic!(),
    };
  }

  #[test]
  fn test_two_puts_and_takes() {
    // three card take
    let take = vec!(deck::ACE_OF_HEARTS, deck::TWO_OF_HEARTS, deck::THREE_OF_HEARTS);
    let mut waste = waste::Waste::new();
    waste.put(take);
    let take = vec!(deck::FOUR_OF_HEARTS, deck::FIVE_OF_HEARTS, deck::SIX_OF_HEARTS);
    waste.put(take);
    check_taken(waste.take(), deck::SIX_OF_HEARTS);
    check_taken(waste.take(), deck::FIVE_OF_HEARTS);
    check_taken(waste.take(), deck::FOUR_OF_HEARTS);
    check_taken(waste.take(), deck::THREE_OF_HEARTS);
    check_taken(waste.take(), deck::TWO_OF_HEARTS);
    check_taken(waste.take(), deck::ACE_OF_HEARTS);

    let taken = waste.take();
    assert_that!(taken.is_none(), is(true));
    assert_that!(waste.cards.is_empty(), is(true));
  }
}
