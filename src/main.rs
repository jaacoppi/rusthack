pub mod creatures;
pub mod dice;
mod input;

use args::{Args, ArgsError};
use creatures::*;
use input::{read_keypress, read_line};

use std::env;
use std::process::exit;

fn main() {
    handle_args();

    let name = read_name();

    let mut user = Creature::new_random(name);
    let mut enemy = Creature::new_random(String::from("Pahis"));

    loop {
        println!("You:\t {}", user.format_stats());
        println!("Enemy:\t {}", enemy.format_stats());
        println!("Press:");
        println!("y: fight");
        match read_keypress() {
            'y' => user.attack(&mut enemy),
            key => println!("A coward, eh? You pressed: {}", key),
        };
        check_deaths(&user, &enemy);
    }
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

fn check_deaths(user: &Creature, enemy: &Creature) -> bool {
    if !user.is_alive() {
        println!("You died. The game has ended!");
        exit(0);
    }
    if !enemy.is_alive() {
        println!("You have killed the enemy. TODO: new enemy, new fight!");
        return true;
    }
    false
}