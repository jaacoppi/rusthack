pub mod creatures;
pub mod dice;
pub mod input;
pub mod stats;

use args::{Args, ArgsError};
use creatures::*;
use input::{read_keypress, read_line};
use std::env;
use std::process::exit;

fn main() {
    handle_args();

    game_loop();
}

fn read_name() -> String {
    loop {
        println!("What do they call you, warrior?");

        let input = read_line(5);
        if input.is_err() {
            println!(
                "{}: That name is too long for a warrior!",
                input.unwrap_err()
            );
            continue;
        }
        break input.unwrap();
    }
}

fn handle_args() {
    let args_string: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_string.iter().map(|s| &**s).collect();
    match parse_args(&args) {
        Ok(()) => true,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };
}

fn parse_args(input: &Vec<&str>) -> Result<(), ArgsError> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let mut args = Args::new(name, description);
    args.flag("h", "help", "Print the usage menu");
    args.flag("v", "version", "Print version information");

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        exit(0);
    }

    let ver = args.value_of("version")?;
    if ver {
        println!("{}", version);
        exit(0);
    }

    Ok(())
}

fn check_deaths(user: &Creature, enemy: &Creature) -> Option<&'static str> {
    if !user.is_alive() {
        return Some("user");
    }
    if !enemy.is_alive() {
        println!("You have killed the enemy.");
        return Some("enemy");
    }
    None
}

fn game_loop() {
    let name = read_name();
    let mut user = Creature::new_random(name);

    loop {
        println!("What now?");
        println!("f: start a fight with a random enemy");
        println!("q: quit game");
        match read_keypress() {
            'f' => fight_loop(&mut user),
            'q' => exit(0),
            _ => todo!(),
        };
    }
}
fn fight_loop(user: &mut Creature) {
    let mut enemy = Creature::new_random(String::from("Pahis"));
    loop {
        println!("You:\t {}", user.format_stats());
        println!("Enemy:\t {}", enemy.format_stats());
        println!("Press:");
        println!("a: attack");
        println!("r: run");
        match read_keypress() {
            'a' => user.attack(&mut enemy),
            'r' => user.run(&mut enemy),
            key => println!("A coward, eh? You pressed: {}", key),
        };
        match check_deaths(&user, &enemy) {
            Some("user") => {
                println!("You died. The game has ended!");
                exit(0);
            }
            Some("enemy") => {
                user.stats.add_kill();
                return;
            }
            _ => (),
        };
    }
}
