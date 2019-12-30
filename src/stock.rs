use crate::card;
use std::cmp;
use std::fmt;

pub struct Stock {
  cards: Vec<&'static card::Card>
}

impl Stock {
  pub fn new(deck: Vec<&'static card::Card>) -> Stock {
    Stock{cards: deck}
  }

  pub fn size(&self) -> usize {
    self.cards.len()
  }

  pub fn is_empty(&self) -> bool {
    self.cards.is_empty()
  }

  pub fn take(&mut self, num: u8) -> Option<Vec<&'static card::Card>> {
    if self.is_empty() {
      None
    }
    else {
      let len = cmp::min(num as usize, self.cards.len());
      let mut taken :Vec<&'static card::Card> = Vec::new();
      for _ in 0..len {
        if let Some(card) = self.cards.pop() { taken.push(card) }
      }
      Some(taken)
    }
  }

  pub fn refresh(&mut self, waste: &[&'static card::Card]) {
    self.cards.clear();
    for card in waste.iter() {
      self.cards.insert(0, card);
    }
  }
}

impl fmt::Display for Stock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_empty() {
      write!(f, "{}", card::DISPLAY_EMPTY)
    }
    else {
      write!(f, "{}", card::DISPLAY_CARD_BACK)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::card;
  use crate::deck;
  use crate::stock;
  use hamcrest2::prelude::*;

  fn create_five_card_stock() -> stock::Stock {
    let mut test_deck = Vec::new();
    test_deck.push(deck::ACE_OF_HEARTS);
    test_deck.push(deck::TWO_OF_HEARTS);
    test_deck.push(deck::THREE_OF_HEARTS);
    test_deck.push(deck::FOUR_OF_HEARTS);
    test_deck.push(deck::FIVE_OF_HEARTS);
    stock::Stock::new(test_deck)
  }

  fn check_single_taken(opt: Option<Vec<&'static card::Card>>, card: &'static card::Card) {
    match opt {
      Some(taken) => assert_that!(taken[0], eq(card)),
      None => panic!()
    }
  }

  #[test]
  fn test_create_stock() {
    let stock = create_five_card_stock();
    assert_that!(stock.is_empty(), is(false));
  }

  #[test]
  fn test_display_not_empty() {
    let stock = create_five_card_stock();
    assert_that!(format!("{}", stock), eq("{###}"));
  }

  #[test]
  fn test_display_empty() {
    let mut stock = create_five_card_stock();
    check_single_taken(stock.take(1), deck::FIVE_OF_HEARTS);
    check_single_taken(stock.take(1), deck::FOUR_OF_HEARTS);
    check_single_taken(stock.take(1), deck::THREE_OF_HEARTS);
    check_single_taken(stock.take(1), deck::TWO_OF_HEARTS);
    check_single_taken(stock.take(1), deck::ACE_OF_HEARTS);
    assert_that!(stock.is_empty(), is(true));
    assert_that!(format!("{}", stock), eq("{   }"));
  }

  #[test]
  fn test_take_one() {
    let mut stock = create_five_card_stock();
    check_single_taken(stock.take(1), deck::FIVE_OF_HEARTS);
    check_single_taken(stock.take(1), deck::FOUR_OF_HEARTS);
    check_single_taken(stock.take(1), deck::THREE_OF_HEARTS);
    check_single_taken(stock.take(1), deck::TWO_OF_HEARTS);
    check_single_taken(stock.take(1), deck::ACE_OF_HEARTS);
    let opt = stock.take(1);
    assert_that!(opt.is_none(), is(true));
    assert_that!(stock.is_empty(), is(true));
  }

  #[test]
  fn test_take_three() {
    let mut stock = create_five_card_stock();
    let opt = stock.take(3);
    match opt {
      Some(taken) => {
        assert_that!(taken.len(), eq(3));
        // top of take
        assert_that!(taken[2], eq(deck::THREE_OF_HEARTS));
        assert_that!(taken[1], eq(deck::FOUR_OF_HEARTS));
        assert_that!(taken[0], eq(deck::FIVE_OF_HEARTS));
      },
      None => panic!(),
    };
    let opt = stock.take(3);
    match opt {
      Some(taken) => {
        assert_that!(taken.len(), eq(2));
        // top of take
        assert_that!(taken[1], eq(deck::ACE_OF_HEARTS));
        assert_that!(taken[0], eq(deck::TWO_OF_HEARTS));
      },
      None => panic!(),
    };
    assert_that!(stock.is_empty(), is(true));
  }

  #[test]
  fn test_refresh() {
    let mut stock = create_five_card_stock();
    let opt = stock.take(5);
    match opt {
      Some(taken) => {
        assert_that!(taken.len(), eq(5));
        // top of take
        assert_that!(taken[4], eq(deck::ACE_OF_HEARTS));
        assert_that!(taken[3], eq(deck::TWO_OF_HEARTS));
        assert_that!(taken[2], eq(deck::THREE_OF_HEARTS));
        assert_that!(taken[1], eq(deck::FOUR_OF_HEARTS));
        assert_that!(taken[0], eq(deck::FIVE_OF_HEARTS));
        assert_that!(stock.is_empty(), is(true));

        stock.refresh(&taken);
        assert_that!(stock.cards.len(), eq(5));
        let opt = stock.take(5);
        match opt {
          Some(taken) => {
            assert_that!(taken.len(), eq(5));
            // top of take
            assert_that!(taken[4], eq(deck::ACE_OF_HEARTS));
            assert_that!(taken[3], eq(deck::TWO_OF_HEARTS));
            assert_that!(taken[2], eq(deck::THREE_OF_HEARTS));
            assert_that!(taken[1], eq(deck::FOUR_OF_HEARTS));
            assert_that!(taken[0], eq(deck::FIVE_OF_HEARTS));
            assert_that!(stock.is_empty(), is(true));
          },
          None => panic!()
        };
      },
      None => panic!()
    };
  }
}