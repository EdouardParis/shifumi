extern crate clap;
mod shifumi;

use clap::{App, SubCommand};
use std::io::{self, Write};

fn main() {
    let matches = App::new("shifumi")
        .version("0.1.0")
        .author("Edouard Paris")
        .about("shifumi game written in Rust")
        .subcommand(SubCommand::with_name("play").about("start a new game"))
        .get_matches();

    if matches.is_present("play") {
        println!(
            "game is started, please choose between:\np: paper | s: scissors | r: rock | q: quit"
        );

        let mut input = String::new();
        let mut bot_play = shifumi::Play::new();
        let mut scoreboard = (0, 0);
        let mut history: Vec<shifumi::Play> = Vec::new();
        let mut tree = shifumi::Tree::new();

        loop {
            input.clear();
            print!("you: ");
            io::stdout().flush().expect("flush failed");
            io::stdin().read_line(&mut input).expect(
                "failed to read from stdin",
            );

            if input.trim() == "q" {
                break;
            }

            if input.trim() == "d" {
                display_tree(&tree);
                continue;
            }

            let mut play = shifumi::Play::new();
            match string_to_play(&input) {
                Some(p) => play = p,
                None => println!("please choose between: p | s | r | q"),
            }

            // Bot play
            update_scoreboard(&mut scoreboard, &play, &bot_play);
            let (win, lose) = scoreboard;
            println!("bot: {} | W:{} L: {}", play_to_string(bot_play), win, lose);

            // Bot try to predict next play
            history.push(play);
            tree.update(&history);
            bot_play = match tree.predict(&history) {
                Some(p) => p,
                None => shifumi::Play::new(),
            };
        }
    }
}

fn update_scoreboard(scoreboard: &mut (i32, i32), play: &shifumi::Play, bot_play: &shifumi::Play) {
    let (ref mut win, ref mut lose) = *scoreboard;
    if &play.beat() == bot_play {
        *lose = *lose + 1;
        return;
    }

    if &bot_play.beat() == play {
        *win = *win + 1;
    }
}

fn play_to_string(play: shifumi::Play) -> String {
    match play {
        shifumi::Play::Paper => String::from("p"),
        shifumi::Play::Rock => String::from("r"),
        shifumi::Play::Scissors => String::from("s"),
    }
}

fn string_to_play(play: &String) -> Option<shifumi::Play> {
    match play.trim() {
        "p" => Some(shifumi::Play::Paper),
        "r" => Some(shifumi::Play::Rock),
        "s" => Some(shifumi::Play::Scissors),
        _ => None,
    }
}

fn display_tree(tree: &shifumi::Tree) {
    match *tree {
        shifumi::Tree::Empty => print!("E "),
        shifumi::Tree::Leaf {
            ref scissors,
            ref paper,
            ref rock,
        } => {
            print!("(S:");
            display_tree(scissors);
            print!("R:");
            display_tree(rock);
            print!("P:");
            display_tree(paper);
            print!(")");
        }
    }
}
