use rand::seq::IndexedRandom;
use std::io;
use std::vec;

pub fn beats(player: &str, foe: &str) -> bool {
    if player == "g" && foe == "w" {
        true
    } else if player == "w" && foe == "f" {
        true
    } else if player == "f" && foe == "g" {
        true
    } else {
        false
    }
}
