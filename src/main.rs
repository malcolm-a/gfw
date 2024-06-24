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

fn main() {
    let mut play = true;
    let types = vec!["g", "f", "w"];
    let mut wins = 0;
    let mut losses = 0;
    let mut turn = 0;

    println!("********* Grass, Fire, Water *********");

    while play {
        turn += 1;
        println!("\n\n*** Round {} ***", turn);
        println!("Grass, Fire or Water ? (G/F/W)");

        let mut choice = String::new();
        loop {
            choice.clear();
            io::stdin().read_line(&mut choice).unwrap();
            choice = choice.to_lowercase().chars().next().unwrap().to_string();
            if types.contains(&choice.as_str()) {
                break;
            } else {
                println!("Invalid choice. [G]rass, [F]ire or [W]ater?");
            }
        }
        let computer = types.choose(&mut rand::thread_rng()).unwrap();
        println!("You      : {}\nComputer : {}", choice, computer);

        if beats(&choice, &computer) {
            println!("You win!");
            wins += 1;
        } else if beats(&computer, &choice) {
            println!("Computer wins!");
            losses += 1;
        } else {
            println!("It's a draw!")
        }

        println!("\n--- SCORE ---\nWins   : {}\nLosses : {}", wins, losses);
        println!("\nContinue ? (y/n)");

        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        choice = choice.to_lowercase().chars().next().unwrap().to_string();
        if choice == "n" || choice == "q" {
            play = false;
        }
    }
}
