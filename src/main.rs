mod dice;
mod creatures;
extern crate diceroll;

use creatures::*;

fn main() {


    let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1, damage: String::from("2d4+1")};
    let user = Creature {name: String::from("Juho"), symbol: '@', hp: 100, item: item};

    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    dice::throw(user.item.damage);
}
