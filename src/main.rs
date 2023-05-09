use diceroll::*;
extern crate diceroll;

fn main() {
    let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1};
    let user = Creature {name: String::from("Juho"), symbol: '@', hp: 100, item: item};

    println!("Hello, world! {0}, {1}, {2} with a {3} with {4} uses", user.name, user.symbol, user.hp, user.item.name, user.item.uses);
    diceroller(2,10);
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
    uses: i32
}

enum ItemType {
    TOOL,
    WEAPON
}



fn diceroller(dice: i32, sides: i32) {
    let modifier = 2;
    let result = roll(DiceRoll::new()
        .dice(dice)
        .sides(sides)
        .modifier(modifier));
    println!("We rolled {}d{}+{}, which yielded a total of {}.", dice, sides, modifier, result.total);
}