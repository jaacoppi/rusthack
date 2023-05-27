pub mod dice;
pub mod creatures;

use creatures::*;

fn main() {

    let name = loop {
        println!("What do they call you, warrior?");
        let input = read_input(5);
        if input.is_err() {
            println!("{}: That name is too long for a warrior!", input.unwrap_err());
            continue;
        }
        break input.unwrap();
    };

    let mut user = Creature::new_random(name);
    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    let mut enemy = Creature::new_random(String::from("Pahis"));
    println!("Enemy:! {0}, {1}, {2} with a {3} with {4} uses", enemy.name, user.symbol, enemy.hp, enemy.item.name, enemy.item.uses);
    user.attack(&mut enemy);

}

fn read_input(max_length: usize) -> Result<String, &'static str> {
    let mut input = String::new();
    let len = std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.replace("\n", "");

    if len -1 <= max_length {
        Ok(input)
    } else {
        Err("Input too long!")
    }
}