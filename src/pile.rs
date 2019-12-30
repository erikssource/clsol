use crate::card;
use std::fmt::Write;

pub struct Pile {
  cards: Vec<card::CardState>,
}

impl Pile {
  pub fn new() -> Pile {
    Pile{cards: Vec::new()}
  }

  pub fn display_row(&self, row: u8) -> String {
    if row as usize >= self.cards.len() {
      String::from(card::DISPLAY_BLANK)
    } else {   
      let mut row_display = String::new();
      write!(&mut row_display, "{}", self.cards[row as usize]).expect("Error occurred while trying to write row to string.");
      row_display
    }
  }

  pub fn len(&self) -> u8 {
    self.cards.len() as u8
  }

  // TODO: cut_king and cut_pile should both call a common function with a closure
  // for the condition.
  pub fn cut_king(&mut self, cut: &mut Vec<card::CardState>) {
    let mut index: i8 = -1;
    let mut i = self.cards.len();
    for card_state in self.cards.iter().rev() {
      i -= 1;
      if card_state.is_visible() 
        && card_state.card.is_king() {
          index = i as i8;
          break;          
      }
    }
    if index >= 0 {
      let mut cut_col = self.cards.split_off(index as usize);
      let num_cut = cut_col.len();
      if num_cut > 0 {
        self.flip_top();
        cut.append(&mut cut_col);
      }
    }
  }

  pub fn cut_pile(&mut self, cmp_card: &card::Card, cut: &mut Vec<card::CardState>) {
    let mut index: i8 = -1;
    let mut i = self.cards.len();
    for card_state in self.cards.iter().rev() {
      i -= 1;
      if card_state.is_visible() 
        && card_state.card.is_different_color(cmp_card)
        && card_state.card.is_one_more(cmp_card) {
          index = i as i8;
          break;          
      }
    }
    if index >= 0 {
      let mut cut_col = self.cards.split_off(index as usize);
      let num_cut = cut_col.len();
      if num_cut > 0 {
        self.flip_top();
        cut.append(&mut cut_col);
      }
    }
  }

  pub fn add_cut(&mut self, cut: &mut Vec<card::CardState>) {
    self.cards.append(cut);
  }

  fn get_top_index(&self) -> usize {
    self.cards.len() - 1
  }

  pub fn get_top(&self) -> Option<&'static card::Card> {
    if !self.cards.is_empty() && self.cards[self.get_top_index()].is_visible() {
      Some(self.cards[self.get_top_index()].card)
    }
    else {
      None
    }
  }

  pub fn is_empty(&self) -> bool {
    self.cards.is_empty()
  }

  pub fn populate(&mut self, card: &'static card::Card) {
    self.cards.push(card::CardState::new(card, false));
  }

  pub fn flip_top(&mut self) {
    if !self.is_empty() {
      let top_index = self.get_top_index();
      self.cards[top_index].make_visible();
    }
  }

  pub fn can_add(&self, card: &'static card::Card) -> bool {
    let top = self.get_top();
    match top {
      Some(top_card) => {
        top_card.is_different_color(card) && top_card.is_one_less(card)
      },
      None => self.cards.is_empty() && card.value == card::Value::King,
    }
  }

  pub fn add_card(&mut self, card: &'static card::Card) {
    self.cards.push(card::CardState::new(card, true));
  }

  pub fn take(&mut self) -> Option<&'static card::Card> {
    match self.cards.pop() {
      Some(taken_card) => {
        self.flip_top();
        Some(taken_card.card)
      },
      None => None
    }
  }
}

#[cfg(test)]
mod tests {
  use hamcrest2::prelude::*;
  use crate::deck;
  use crate::card::*;
  use crate::pile::*;

  fn setup() -> Pile {
    let mut col = Pile::new();
    col.populate(deck::QUEEN_OF_HEARTS);
    col.populate(deck::NINE_OF_CLUBS);
    col.populate(deck::QUEEN_OF_SPADES);
    col
  }

  fn setup_for_cut() -> Pile {
    let mut col = setup();
    col.populate(deck::TEN_OF_CLUBS);
    col.flip_top();
    col.add_card(deck::NINE_OF_HEARTS);
    col.add_card(deck::EIGHT_OF_SPADES);
    col.add_card(deck::SEVEN_OF_DIAMONDS);
    col.add_card(deck::SIX_OF_CLUBS);
    col
  }

  #[test]
  fn test_add_card() {
    let mut col = setup();
    col.add_card(deck::TEN_OF_CLUBS);
    assert_that!(col.cards.len(), eq(4));
    match col.get_top() {
      Some(top_card) => {
        assert_that!(top_card, eq(deck::TEN_OF_CLUBS));
      },
      None => panic!(),
    }    
  }

  #[test]
  fn test_cut_pile_false_no_match() {
    let mut col = setup_for_cut();
    let mut cut_vec = Vec::new();
    col.cut_pile(deck::TWO_OF_HEARTS, &mut cut_vec);
    assert_that!(cut_vec.len(), eq(0));
  }

  #[test]
  fn test_cut_pile_false_invisible() {
    let mut col = setup_for_cut();
    let mut cut_vec = Vec::new();
    col.cut_pile(deck::TWO_OF_HEARTS, &mut cut_vec);
    assert_that!(cut_vec.len(), eq(0));
  }

  #[test]
  fn test_cut_pile() {
    let mut col = setup_for_cut();
    let mut cut_vec = Vec::new();
    col.cut_pile(deck::JACK_OF_HEARTS, &mut cut_vec);
    assert_that!(cut_vec.len(), eq(5));
  }

  #[test]
  fn test_cut_pile_and_add() {
    let mut col1 = Pile::new();
    col1.add_card(deck::JACK_OF_HEARTS);
    let mut col2 = setup_for_cut();
    let mut cut_vec = Vec::new();
    col2.cut_pile(deck::JACK_OF_HEARTS, &mut cut_vec);
    assert_that!(cut_vec.len(), eq(5));
    col1.add_cut(&mut cut_vec);
    assert_that!(col1.cards.len(), eq(6));
  }

  #[test]
  fn test_cut_one() {
    let mut col = setup_for_cut();
    let mut cut_vec = Vec::new();
    col.cut_pile(deck::SEVEN_OF_HEARTS, &mut cut_vec);
    assert_that!(cut_vec.len(), eq(1));
  }

  #[test]
  fn test_create_pile() {
    let col = Pile::new();
    assert_that!(col.is_empty(), is(true));
  }

  #[test]
  fn test_populate() {
    let mut col = setup();
    col.flip_top();
    assert_that!(col.is_empty(), is(false));

    match col.get_top() {
      Some(card) => {
        assert_that!(card.suit.code(), eq('S'));
        assert_that!(card.value.rank(), eq(Value::Queen.rank()));
        assert_that!(col.cards.len(), eq(3));
        assert_that!(col.get_top_index(), eq(2));
      },
      None => panic!(),
    }
  }

  #[test]
  fn test_can_add_true() {
    let mut col = setup();
    col.flip_top();
    assert_that!(col.can_add(deck::JACK_OF_HEARTS), is(true));
    assert_that!(col.can_add(deck::JACK_OF_DIAMONDS), is(true));
  }

  #[test]
  fn test_can_add_false_bc_suit() {
    let mut col = setup();
    col.flip_top();
    assert_that!(col.can_add(deck::JACK_OF_CLUBS), is(false));
    assert_that!(col.can_add(deck::JACK_OF_SPADES), is(false));
  }

  #[test]
  fn test_can_add_false_bc_value() {
    let mut col = setup();
    col.flip_top();
    assert_that!(col.can_add(deck::KING_OF_HEARTS), is(false));
    assert_that!(col.can_add(deck::TEN_OF_HEARTS), is(false));
    assert_that!(col.can_add(deck::QUEEN_OF_HEARTS), is(false));
  }

  #[test]
  fn test_can_add_to_empty_true() {
    let col = Pile::new();
    let king_of_hearts :&'static Card = &Card{suit: Suit::Heart, value: Value::King};
    assert_that!(col.can_add(king_of_hearts), is(true));
  }

  #[test]
  fn test_can_add_to_empty_false() {
    let col = Pile::new();
    let ten_of_hearts :&'static Card = &Card{suit: Suit::Heart, value: Value::Ten};
    assert_that!(col.can_add(ten_of_hearts), is(false));
  }

  #[test]
  fn test_take_valid() {
    let mut col = setup();
    col.flip_top();
    let taken = col.take();
    match taken {
      Some(card) => {
        assert_that!(card.suit.code(), eq('S'));
        assert_that!(card.value.rank(), eq(Value::Queen.rank()));
        assert_that!(col.cards.len(), eq(2));
      },
      None => {
        panic!();
      },
    }
  }

  #[test]
  fn test_take_invalid() {
    let mut col = Pile::new();
    let taken = col.take();
    assert_that!(taken, none());
  }

  #[test]
  fn test_row_display() {
    let mut col = setup();
    col.flip_top();
    let row_0 = col.display_row(0);
    let row_1 = col.display_row(1);
    let row_2 = col.display_row(2);
    let row_3 = col.display_row(3);

    assert_that!(row_0, eq("{###}"));
    assert_that!(row_1, eq("{###}"));
    assert_that!(row_2, eq("[Q.S]"));
    assert_that!(row_3, eq("     "));
  }
}
