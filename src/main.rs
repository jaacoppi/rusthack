pub mod dice;
pub mod creatures;

use creatures::*;

fn main() {
    let mut user = Creature::new_random(String::from("Juho"));
    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    let mut enemy = Creature::new_random(String::from("Pahis"));
    println!("Enemy:! {0}, {1}, {2} with a {3} with {4} uses", enemy.name, user.symbol, enemy.hp, enemy.item.name, enemy.item.uses);
    user.attack(&mut enemy);



    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    dice::throw(&user.item.damage);
}
