#![allow(dead_code)]
use crate::card::Card;
use crate::card::Suit;
use crate::card::Value;

pub const FULL_DECK: [Card; 52] = [
  Card { suit: Suit::Heart,   value: Value::Ace},
  Card { suit: Suit::Heart,   value: Value::Two},
  Card { suit: Suit::Heart,   value: Value::Three},
  Card { suit: Suit::Heart,   value: Value::Four},
  Card { suit: Suit::Heart,   value: Value::Five},
  Card { suit: Suit::Heart,   value: Value::Six},
  Card { suit: Suit::Heart,   value: Value::Seven},
  Card { suit: Suit::Heart,   value: Value::Eight},
  Card { suit: Suit::Heart,   value: Value::Nine},
  Card { suit: Suit::Heart,   value: Value::Ten},
  Card { suit: Suit::Heart,   value: Value::Jack},
  Card { suit: Suit::Heart,   value: Value::Queen},
  Card { suit: Suit::Heart,   value: Value::King},
  Card { suit: Suit::Diamond, value: Value::Ace},
  Card { suit: Suit::Diamond, value: Value::Two},
  Card { suit: Suit::Diamond, value: Value::Three},
  Card { suit: Suit::Diamond, value: Value::Four},
  Card { suit: Suit::Diamond, value: Value::Five},
  Card { suit: Suit::Diamond, value: Value::Six},
  Card { suit: Suit::Diamond, value: Value::Seven},
  Card { suit: Suit::Diamond, value: Value::Eight},
  Card { suit: Suit::Diamond, value: Value::Nine},
  Card { suit: Suit::Diamond, value: Value::Ten},
  Card { suit: Suit::Diamond, value: Value::Jack},
  Card { suit: Suit::Diamond, value: Value::Queen},
  Card { suit: Suit::Diamond, value: Value::King},
  Card { suit: Suit::Spade,   value: Value::Ace},
  Card { suit: Suit::Spade,   value: Value::Two},
  Card { suit: Suit::Spade,   value: Value::Three},
  Card { suit: Suit::Spade,   value: Value::Four},
  Card { suit: Suit::Spade,   value: Value::Five},
  Card { suit: Suit::Spade,   value: Value::Six},
  Card { suit: Suit::Spade,   value: Value::Seven},
  Card { suit: Suit::Spade,   value: Value::Eight},
  Card { suit: Suit::Spade,   value: Value::Nine},
  Card { suit: Suit::Spade,   value: Value::Ten},
  Card { suit: Suit::Spade,   value: Value::Jack},
  Card { suit: Suit::Spade,   value: Value::Queen},
  Card { suit: Suit::Spade,   value: Value::King},
  Card { suit: Suit::Club,    value: Value::Ace},
  Card { suit: Suit::Club,    value: Value::Two},
  Card { suit: Suit::Club,    value: Value::Three},
  Card { suit: Suit::Club,    value: Value::Four},
  Card { suit: Suit::Club,    value: Value::Five},
  Card { suit: Suit::Club,    value: Value::Six},
  Card { suit: Suit::Club,    value: Value::Seven},
  Card { suit: Suit::Club,    value: Value::Eight},
  Card { suit: Suit::Club,    value: Value::Nine},
  Card { suit: Suit::Club,    value: Value::Ten},
  Card { suit: Suit::Club,    value: Value::Jack},
  Card { suit: Suit::Club,    value: Value::Queen},
  Card { suit: Suit::Club,    value: Value::King}
];

pub const ACE_OF_HEARTS:   &Card = &FULL_DECK[0];
pub const TWO_OF_HEARTS:   &Card = &FULL_DECK[1];
pub const THREE_OF_HEARTS: &Card = &FULL_DECK[2];
pub const FOUR_OF_HEARTS:  &Card = &FULL_DECK[3];
pub const FIVE_OF_HEARTS:  &Card = &FULL_DECK[4];
pub const SIX_OF_HEARTS:   &Card = &FULL_DECK[5];
pub const SEVEN_OF_HEARTS: &Card = &FULL_DECK[6];
pub const EIGHT_OF_HEARTS: &Card = &FULL_DECK[7];
pub const NINE_OF_HEARTS:  &Card = &FULL_DECK[8];
pub const TEN_OF_HEARTS:   &Card = &FULL_DECK[9];
pub const JACK_OF_HEARTS:  &Card = &FULL_DECK[10];
pub const QUEEN_OF_HEARTS: &Card = &FULL_DECK[11];
pub const KING_OF_HEARTS:  &Card = &FULL_DECK[12];

pub const ACE_OF_DIAMONDS:   &Card = &FULL_DECK[13];
pub const TWO_OF_DIAMONDS:   &Card = &FULL_DECK[14];
pub const THREE_OF_DIAMONDS: &Card = &FULL_DECK[15];
pub const FOUR_OF_DIAMONDS:  &Card = &FULL_DECK[16];
pub const FIVE_OF_DIAMONDS:  &Card = &FULL_DECK[17];
pub const SIX_OF_DIAMONDS:   &Card = &FULL_DECK[18];
pub const SEVEN_OF_DIAMONDS: &Card = &FULL_DECK[19];
pub const EIGHT_OF_DIAMONDS: &Card = &FULL_DECK[20];
pub const NINE_OF_DIAMONDS:  &Card = &FULL_DECK[21];
pub const TEN_OF_DIAMONDS:   &Card = &FULL_DECK[22];
pub const JACK_OF_DIAMONDS:  &Card = &FULL_DECK[23];
pub const QUEEN_OF_DIAMONDS: &Card = &FULL_DECK[24];
pub const KING_OF_DIAMONDS:  &Card = &FULL_DECK[25];

pub const ACE_OF_SPADES:   &Card = &FULL_DECK[26];
pub const TWO_OF_SPADES:   &Card = &FULL_DECK[27];
pub const THREE_OF_SPADES: &Card = &FULL_DECK[28];
pub const FOUR_OF_SPADES:  &Card = &FULL_DECK[29];
pub const FIVE_OF_SPADES:  &Card = &FULL_DECK[30];
pub const SIX_OF_SPADES:   &Card = &FULL_DECK[31];
pub const SEVEN_OF_SPADES: &Card = &FULL_DECK[32];
pub const EIGHT_OF_SPADES: &Card = &FULL_DECK[33];
pub const NINE_OF_SPADES:  &Card = &FULL_DECK[34];
pub const TEN_OF_SPADES:   &Card = &FULL_DECK[35];
pub const JACK_OF_SPADES:  &Card = &FULL_DECK[36];
pub const QUEEN_OF_SPADES: &Card = &FULL_DECK[37];
pub const KING_OF_SPADES:  &Card = &FULL_DECK[38];

pub const ACE_OF_CLUBS:   &Card = &FULL_DECK[39];
pub const TWO_OF_CLUBS:   &Card = &FULL_DECK[40];
pub const THREE_OF_CLUBS: &Card = &FULL_DECK[41];
pub const FOUR_OF_CLUBS:  &Card = &FULL_DECK[42];
pub const FIVE_OF_CLUBS:  &Card = &FULL_DECK[43];
pub const SIX_OF_CLUBS:   &Card = &FULL_DECK[44];
pub const SEVEN_OF_CLUBS: &Card = &FULL_DECK[45];
pub const EIGHT_OF_CLUBS: &Card = &FULL_DECK[46];
pub const NINE_OF_CLUBS:  &Card = &FULL_DECK[47];
pub const TEN_OF_CLUBS:   &Card = &FULL_DECK[48];
pub const JACK_OF_CLUBS:  &Card = &FULL_DECK[49];
pub const QUEEN_OF_CLUBS: &Card = &FULL_DECK[50];
pub const KING_OF_CLUBS:  &Card = &FULL_DECK[51];