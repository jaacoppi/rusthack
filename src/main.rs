pub mod creatures;
pub mod dice;

use args::{Args, ArgsError};
use creatures::*;

use std::env;
use std::process::exit;

fn main() {
    handle_args();

    let name = read_name();

    let mut user = Creature::new_random(name);
    println!(
        "{0}, {1} hp with a {2} with {3} uses",
        user.name, user.hp, user.item.name, user.item.uses
    );
    let mut enemy = Creature::new_random(String::from("Pahis"));
    println!(
        "Enemy: {0}, {1} hp with a {2} with {3} uses",
        enemy.name, enemy.hp, enemy.item.name, enemy.item.uses
    );
    user.attack(&mut enemy);
}

fn read_input(max_length: usize) -> Result<String, &'static str> {
    let mut input = String::new();
    let len = std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input = input.replace('\n', "");

    if len - 1 <= max_length {
        Ok(input)
    } else {
        Err("Input too long!")
    }
}

fn read_name() -> String {
    loop {
        println!("What do they call you, warrior?");

        let args_string: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_string.iter().map(|s| &**s).collect();
    match parse_args(&args) {
        Ok(()) => true,
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };
        let input = read_input(5);
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
    let _version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let mut args = Args::new(name, description);
    args.flag("h", "help", "Print the usage menu");
    args.flag("v", "version", "Print version information");

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
        return Ok(());
    }

    Ok(())
}