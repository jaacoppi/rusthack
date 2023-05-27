pub mod dice;
pub mod creatures;

use creatures::*;

fn main() {
    println!("What do they call you, warrior?");

    let name = read_name();

    let mut user = Creature::new_random(name);
    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    let mut enemy = Creature::new_random(String::from("Pahis"));
    println!("Enemy:! {0}, {1}, {2} with a {3} with {4} uses", enemy.name, user.symbol, enemy.hp, enemy.item.name, enemy.item.uses);
    user.attack(&mut enemy);

}

fn read_name() -> String {
    let mut name = String::new();
    let _input = std::io::stdin().read_line(&mut name).expect("Failed to read line");
    name = name.replace("\n", "");

    name
}