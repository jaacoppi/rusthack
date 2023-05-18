mod dice;
extern crate diceroll;

fn main() {


    let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1, damage: String::from("2d4+1")};
    let user = Creature {name: String::from("Juho"), symbol: '@', hp: 100, item: item};

    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    dice::throw(user.item.damage);
}


struct Creature {
    name: String,
    symbol: char,
    hp: i32,
    item: Item
}

struct Item {
    kind: ItemType,
    name: String,
    uses: i32,
    damage: String
}

enum ItemType {
    TOOL,
    WEAPON
}
