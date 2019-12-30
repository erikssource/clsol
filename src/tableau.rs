use arr_macro::arr;
use std::cmp;

use crate::card;
use crate::pile;

pub struct Tableau {
  pub piles: [pile::Pile; 7]
}

impl Tableau {
  pub fn new() -> Tableau {
    Tableau{piles: arr![pile::Pile::new(); 7]}
  }

  pub fn display(&self) {
    let rows: u8 = self.max_rows();
    println!("||__1__||__2__||__3__||__4__||__5__||__6__||__7__||");
    for row in 0..rows {
      println!("  {}  {}  {}  {}  {}  {}  {}  ", 
        self.piles[0].display_row(row),
        self.piles[1].display_row(row),
        self.piles[2].display_row(row),
        self.piles[3].display_row(row),
        self.piles[4].display_row(row),
        self.piles[5].display_row(row),
        self.piles[6].display_row(row)
      );
    }
  }

  pub fn do_move(&mut self, src_idx: u8, dest_idx: u8) -> Result<(),()> {
    if self.piles[dest_idx as usize].is_empty() {
      let mut cut_vec = Vec::new();
      self.piles[src_idx as usize].cut_king(&mut cut_vec);
      if !cut_vec.is_empty() {
        self.piles[dest_idx as usize].add_cut(&mut cut_vec);
        Ok(())
      }
      else {
        Err(())
      }
    }
    else if let Some(top_card) = self.piles[dest_idx as usize].get_top() {
		  let mut cut_vec = Vec::new();
      self.piles[src_idx as usize].cut_pile(top_card, &mut cut_vec);
      if !cut_vec.is_empty() {
    	  self.piles[dest_idx as usize].add_cut(&mut cut_vec);
		    Ok(())
      }
      else {
	      Err(())
      }
    }
	  else {
		  Err(())
    }
		/*

      match self.piles[dest_idx as usize].get_top() {
        Some(top_card) => {
          let mut cut_vec = Vec::new();
          self.piles[src_idx as usize].cut_pile(top_card, &mut cut_vec);
          if cut_vec.len() > 0 {
            self.piles[dest_idx as usize].add_cut(&mut cut_vec);
            return Ok(());
          }
        },
        None => (),
      }
      return Err(());
    }
    */
  }

  pub fn take(&mut self, index: u8) -> Option<&'static card::Card> {
    self.piles[index as usize].take()
  }

  pub fn get_top(&self, index: u8) -> Option<&'static card::Card> {
    self.piles[index as usize].get_top()
  }

  pub fn add_card(&mut self, index: u8, card: &'static card::Card) {
    self.piles[index as usize].populate(card);
  }

  pub fn flip_all(&mut self) {
    for pile in self.piles.iter_mut() {
      pile.flip_top();
    }
  }

  fn max_rows(&self) -> u8 {
    let mut max_rows: u8 = 0;
    for pile in &self.piles {
      max_rows = cmp::max(max_rows, pile.len());
    };
    max_rows
  }
}

