#[macro_use]
extern crate text_io;
extern crate rssol;

use rssol::Success;
use rssol::Failure;
use rssol::Solitaire;

fn main() {
    let mut sol = Solitaire::new();
    sol.new_game();
    println!("{}\n", sol.display());
    loop {
        println!("\nEnter Command (? for show commands)");
        let input: String = read!();
        match sol.command(&input) {
            Ok(success) => {
                match success {
                    Success::Quit => break,
                    Success::Help(help_text) => {
                        println!("{}\n", help_text);
                    },
                    Success::Retire => {
                        println!("GAME IS LOST\n");
                        println!("{}\n", sol.display());
                    },
                    Success::ValidMove(display) => {
                        println!("{}\n", display);
                    },
                    Success::Victory(display) => {
                        println!("{}\n", display);
                        println!("VICTORY!!!");
                        sol.new_game();
                        println!("{}\n", sol.display());
                    }
                }
            },
            Err(failure) => {
                match failure {
                    Failure::InvalidMove => {
                        println!("Invalid Move!\n");
                        println!("{}\n", sol.display());
                    },
                    Failure::InvalidCommand => {
                        println!("Invalid Command!\n");
                        println!("{}\n", sol.display());
                    }
                }
            }
        }
    }
}
