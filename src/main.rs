mod dice;
mod creatures;
extern crate diceroll;

use creatures::*;

fn main() {
    let mut user = random_creature();
    let mut enemy = random_creature();
    creatures::attack(&mut user, &mut enemy);

    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    dice::throw(&user.item.damage);
}
