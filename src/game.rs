use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card;
use crate::foundation::Foundation;
use crate::stock::Stock;
use crate::tableau::Tableau;
use crate::waste::Waste;
use crate::deck::FULL_DECK;

pub const HEART_FD: u8 = 0;
pub const DIAMOND_FD: u8 = 1;
pub const SPADE_FD: u8 = 2;
pub const CLUB_FD: u8 = 3;

pub struct Game {
  foundations: [Foundation; 4],
  stock: Stock,
  waste: Waste,
  tableau: Tableau,
  turn: u16,
}

impl Game {
  pub fn new() -> Game {
    let mut cards = Vec::new();
    for card in FULL_DECK.iter() {
      cards.push(card);
    }
    cards.shuffle(&mut thread_rng());
    // Create and populate tableau
    let mut tableau = Tableau::new();

    // Deal tableau
    for i in 0..7 {
      for _ in 0..=i {
        match cards.pop() {
          Some(card) => tableau.add_card(i, card),
          None => panic!("Not playing with a full deck"),
        }
      }
    }
    tableau.flip_all();

    Game{ 
      foundations: [
        Foundation::new(card::Suit::Heart),
        Foundation::new(card::Suit::Diamond),
        Foundation::new(card::Suit::Spade),
        Foundation::new(card::Suit::Club),
      ],
      stock: Stock::new(cards),
      waste: Waste::new(),
      tableau,
      turn: 0,
    }
  }

  pub fn victory(&self) -> bool {
    self.foundations[HEART_FD as usize].is_full() 
      && self.foundations[DIAMOND_FD as usize].is_full() 
      && self.foundations[SPADE_FD as usize].is_full() 
      && self.foundations[CLUB_FD as usize].is_full() 
  }

  fn invalid_move(&self) {
    println!("-----------------------------------------------------");
    println!(" INVALID MOVE!");
  }

  pub fn draw_from_stock(&mut self) {
    if self.stock.is_empty() {
      self.stock.refresh(self.waste.get_all());
      self.waste.clear();
    }
    let opt = self.stock.take(1);
    match opt {
      Some(taken) => {
        self.waste.put(taken);
        self.turn += 1;
      },
      None => {
        self.invalid_move();
      }
    }
  }

  pub fn waste_to_foundation(&mut self) {
    let opt = self.waste.get_top();
    match opt {
      Some(top_card) => {
        for foundation in self.foundations.iter_mut() {
          if foundation.can_add(top_card) {
            let pop_opt = self.waste.take();
            if let Some(pop) = pop_opt {
              foundation.add(pop);
              self.turn += 1;
              return;
            }
          }
        }
        self.invalid_move();
      },
      None => self.invalid_move(),
    }
  }

  pub fn auto_finish(&mut self) {
      println!("Not yet implemented");
  }

  pub fn waste_to_pile(&mut self, pile_num: u8) {
    let pile_idx = pile_num - 1;
    if let Some(top_card) = self.waste.get_top() {
      let pile = &mut self.tableau.piles[pile_idx as usize];
      if pile.can_add(top_card) {
        if let Some(popped_card) = self.waste.take() {
          pile.add_card(popped_card);
          self.turn += 1;
          return;
        }
      }
    }
    self.invalid_move();
  }

  pub fn foundation_to_pile(&mut self, foundation_index: u8, pile_index: u8) {
    //TODO: Get rid of duplicate code with wast to pile. Probably need to define trait 
    //      for being able to take a card.
    if let Some(top_card) = self.foundations[foundation_index as usize].get_top() {
      let pile_idx = pile_index - 1;
      let pile = &mut self.tableau.piles[pile_idx as usize];
      if pile.can_add(top_card) {
        if let Some(popped_card) = self.foundations[foundation_index as usize].take() {
          pile.add_card(popped_card);
          self.turn += 1;
          return;
        }
      }
    }
    self.invalid_move();
  }

  pub fn pile_to_foundation(&mut self, pile_num: u8) {
    match self.tableau.get_top(pile_num - 1) {
      Some(top_card) => {
        for foundation in self.foundations.iter_mut() {
          if foundation.can_add(top_card) {
            if let Some(pop) = self.tableau.take(pile_num - 1) {
              foundation.add(pop);
              self.turn += 1;
              return;     
            }
          }
        }
        self.invalid_move();
      },
      None => self.invalid_move(),
    }
  }

  pub fn pile_to_pile(&mut self, src_pile_num: u8, dest_pile_num: u8) {
    let src_idx = src_pile_num - 1;
    let dest_idx = dest_pile_num - 1;
    match self.tableau.do_move(src_idx, dest_idx) {
      Ok(_) => { 
        self.turn += 1;
      },
      Err(_) => {
        self.invalid_move();
      },
    }
  }

  pub fn display(&self) {
    println!("-----------------------------------------------------");
    println!("Turn: {}   Stock: {}   Waste: {}", self.turn, self.stock.size(), self.waste.size());
    println!("    n      k            h      d      s      c");
    println!("  {}  {}        {}  {}  {}  {}",
      self.stock,
      self.waste,
      self.foundations[HEART_FD as usize],
      self.foundations[DIAMOND_FD as usize],
      self.foundations[SPADE_FD as usize],
      self.foundations[CLUB_FD as usize]
    );
    println!();
    self.tableau.display();
  }
}
