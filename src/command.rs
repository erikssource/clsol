use crate::game;

/*
 * Commands 
 * 
 * ?          : ShowHelp
 * r          : Retire
 * n          : DrawFromStock
 * q          : Quit
 * k          : WasteToFoundation
 * a          : AutoFinish
 * k[1-7]     : WasteToPile [pile 0-6] 
 * [1-7]      : PileToFoundation [pile 0-6]
 * [1-7][1-7] : PileToPile [pile 0-6][pile 0-6]
 * h[1-7]     : FoundationToPile[found: 0-3][pile 0-6]
 * d[1-7]     : FoundationToPile[found: 0-3][pile 0-6]
 * s[1-7]     : FoundationToPile[found: 0-3][pile 0-6]
 * c[1-7]     : FoundationToPile[found: 0-3][pile 0-6]
 */

// TODO: Stats, StatsReset
pub enum Command {
  ShowHelp,
  Retire,
  Quit,
  DrawFromStock,
  WasteToFoundation,
  AutoFinish,
  WasteToPile{pile_index: u8},
  PileToFoundation{pile_index: u8},
  PileToPile{src_pile: u8, dest_pile: u8},
  FoundationToPile{foundation_index: u8, pile_index: u8},
}

impl Command {
  pub fn from_string(cmd_str: &str) -> Option<Command> {
    let s = cmd_str.trim().to_lowercase();
    if s.is_empty() {
      None
    }
    else if s.len() == 1 {
      let opt1 = s.parse::<u8>();
      match opt1 {
        Ok(num) => {
          if num >= 1 && num <= 7 {
            Some(Command::PileToFoundation{pile_index: num})
          }
          else {
            None
          }
        } 
        _ => {
          if s == "?" {
            Some(Command::ShowHelp)
          }
          else if s == "r" {
            Some(Command::Retire)
          }
          else if s == "n" {
            Some(Command::DrawFromStock)
          }
          else if s == "q" {
            Some(Command::Quit)
          }
          else if s == "k" {
            Some(Command::WasteToFoundation)
          }
          else {
            None
          }
        }
      }
    }
    else if s.len() == 2 {
      let(s1, s2) = s.split_at(1);
      let opt1 = s1.parse::<u8>();
      match opt1 {
        Ok(num1) => {
          let opt2 = s2.parse::<u8>();
          match opt2 {
            Ok(num2) => {
              if num1 >= 1 && num1 <= 7 && num2 >= 1 && num2 <= 7 {
                Some(Command::PileToPile{src_pile: num1, dest_pile: num2})
              }
              else {
                None
              }
            },
            _ => None
          }
        },
        _ => {
          if s1 == "k" || s1 == "h" || s1 == "d" || s1 == "s" || s1 == "c" {
            let opt2 = s2.parse::<u8>();
            match opt2 {
              Ok(num2) => {
                if num2 >= 1 && num2 <= 7 {
                  match s1 {
                    "h" => Some(Command::FoundationToPile{foundation_index: game::HEART_FD, pile_index: num2}),
                    "d" => Some(Command::FoundationToPile{foundation_index: game::DIAMOND_FD, pile_index: num2}),
                    "s" => Some(Command::FoundationToPile{foundation_index: game::SPADE_FD, pile_index: num2}),
                    "c" => Some(Command::FoundationToPile{foundation_index: game::CLUB_FD, pile_index: num2}),
                    "k" => Some(Command::WasteToPile{pile_index: num2}),
                    _ => None,
                  }
                }
                else {
                  None
                }
              },
              _ => None,
            }
          }
          else {
            None
          }
        }
      }
    }
    else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::command::Command;
  use hamcrest2::prelude::*;
  
  fn enum_type(opt_cmd: &Option<Command>) -> String {
    match opt_cmd {
      Some(cmd) => {     
        #[allow(unused_variables)]   
        match cmd {
          Command::ShowHelp => String::from("ShowHelp"),
          Command::Retire => String::from("Retire"),
          Command::Quit => String::from("Quit"),
          Command::DrawFromStock => String::from("DrawFromStock"),
          Command::WasteToFoundation => String::from("WasteToFoundation"),
          Command::AutoFinish => String::from("AutoFinish"),
          Command::WasteToPile{pile_index} => String::from("WasteToPile"),
          Command::PileToFoundation{pile_index} => String::from("PileToFoundation"),
          Command::PileToPile{src_pile, dest_pile} => String::from("PileToPile"),
          Command::FoundationToPile{foundation_index, pile_index} => String::from("FoundationToPile"),
        }
      },
      None => String::from("")
    }
  }

  #[test]
  fn test_cmd_type() {
    let cmd = Command::from_string("?");
    assert_that!(enum_type(&cmd), eq("ShowHelp"));

    let cmd = Command::from_string("r");
    assert_that!(enum_type(&cmd), eq("Retire"));

    let cmd = Command::from_string("n");
    assert_that!(enum_type(&cmd), eq("DrawFromStock"));

    let cmd = Command::from_string("q");
    assert_that!(enum_type(&cmd), eq("Quit"));

    let cmd = Command::from_string("k");
    assert_that!(enum_type(&cmd), eq("WasteToFoundation"));

    // let cmd = Command::from_string("a");
    // assert_that!(enum_type(&cmd), eq("AutoFinish"));

    let cmd = Command::from_string("k1");
    assert_that!(enum_type(&cmd), eq("WasteToPile"));

    let cmd = Command::from_string("2");
    assert_that!(enum_type(&cmd), eq("PileToFoundation"));

    let cmd = Command::from_string("21");
    assert_that!(enum_type(&cmd), eq("PileToPile"));

    let cmd = Command::from_string("i");
    assert_that!(enum_type(&cmd), eq(""));
  }

  #[test]
  fn test_waste_to_pile() {
    let cmd = Command::from_string("k0");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("k1");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::WasteToPile{pile_index} => assert_that!(pile_index, eq(1)),
      _ => panic!(),
    }

    let cmd = Command::from_string("k7");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::WasteToPile{pile_index} => assert_that!(pile_index, eq(7)),
      _ => panic!(),
    }

    let cmd = Command::from_string("k8");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("k9");
    assert_that!(cmd.is_none(), is(true));
  }

  #[test]
  fn test_pile_to_foundation() {
    let cmd = Command::from_string("0");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("1");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToFoundation{pile_index} => assert_that!(pile_index, eq(1)),
      _ => panic!(),
    }

    let cmd = Command::from_string("7");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToFoundation{pile_index} => assert_that!(pile_index, eq(7)),
      _ => panic!(),
    }

    let cmd = Command::from_string("8");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("9");
    assert_that!(cmd.is_none(), is(true));
  }

  #[test]
  fn test_pile_to_pile() {
    let cmd = Command::from_string("01");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("10");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("82");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("78");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("79");
    assert_that!(cmd.is_none(), is(true));

    let cmd = Command::from_string("15");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToPile{src_pile, dest_pile} => {
        assert_that!(src_pile, eq(1));
        assert_that!(dest_pile, eq(5));
      },
      _ => panic!(),
    }

    let cmd = Command::from_string("73");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToPile{src_pile, dest_pile} => {
        assert_that!(src_pile, eq(7));
        assert_that!(dest_pile, eq(3));
      },
      _ => panic!(),
    }

    let cmd = Command::from_string("41");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToPile{src_pile, dest_pile} => {
        assert_that!(src_pile, eq(4));
        assert_that!(dest_pile, eq(1));
      },
      _ => panic!(),
    }

    let cmd = Command::from_string("67");
    assert_that!(cmd.is_none(), is(false));
    match cmd.unwrap() {
      Command::PileToPile{src_pile, dest_pile} => {
        assert_that!(src_pile, eq(6));
        assert_that!(dest_pile, eq(7));
      },
      _ => panic!(),
    }
  }
}