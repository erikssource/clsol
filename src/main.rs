#[macro_use]
extern crate text_io;

mod card;
mod pile;
mod foundation;
mod deck;
mod game;
mod stock;
mod waste;
mod tableau;
mod command;
mod config;

use game::Game;

fn show_help() {
    println!("-----------------------------------------------------");
    println!(" ? . . . . .: Display Help");
    println!(" r          : Retire Current Game");
    println!(" q . . . . .: Quit");
    println!(" n          : Draw Cards from Stock");
    println!(" k . . . . .: Move Card from Waste to Foundation");
    println!(" k[1-7]     : Move Card from Waste to Pile by Number");
    println!(" [1-7] . . .: Move Card from Pile by Number to Foundation");
    println!(" [1-7][1-7] : Move Card from Pile to Pile by Number");
    println!(" h[1-7] . . : Move Card from Hearts Foundation to Pile by Number");
    println!(" d[1-7]     : Move Card from Diamonds Foundation to Pile by Number");
    println!(" s[1-7] . . : Move Card from Spades Foundation to Pile by Number");
    println!(" c[1-7]     : Move Card from Clubs Foundation to Pile by Number");
}

fn main() {
    let mut game = Game::new();
    loop {
        if game.victory() {
          println!("!!!!!");
          println!("VICTORY! CONGRATUATIONS!");
          println!("!!!!!");
          game = Game::new();
        }
        game.display();
        println!("\nEnter Command (? for show commands)");
        let input: String = read!();
        let opt_cmd = command::Command::from_string(&input);
        match opt_cmd {
            Some(cmd) => {
                match cmd {
                    command::Command::Quit => break,
                    command::Command::Retire => {
                        game = Game::new();
                    }
                    command::Command::ShowHelp => {
                        show_help();
                    },
                    command::Command::DrawFromStock => {
                        game.draw_from_stock();
                    },
                    command::Command::WasteToFoundation => {
                        game.waste_to_foundation();
                    },
                    command::Command::WasteToPile{pile_index} => {
                        game.waste_to_pile(pile_index);
                    },
                    command::Command::PileToFoundation{pile_index} => {
                        game.pile_to_foundation(pile_index);
                    },
                    command::Command::PileToPile{src_pile, dest_pile} => {
                        game.pile_to_pile(src_pile, dest_pile);
                    },
                    command::Command::FoundationToPile{foundation_index, pile_index} => {
                        game.foundation_to_pile(foundation_index, pile_index);
                    },
                }
            },
            None => {
                println!("INVALID COMMAND!");
            }
        }
    }
}
