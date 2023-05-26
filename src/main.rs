pub mod dice;
pub mod creatures;

use creatures::*;

fn main() {
    let mut user = Creature::new_random(String::from("Juho"));
    let mut enemy = Creature::new_random(String::from("Pahis"));
    creatures::attack(&mut user, &mut enemy);

    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    dice::throw(&user.item.damage);
}
