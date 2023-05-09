fn main() {
    let item = Item {name: String::from("lockpick"), uses: -1, itemtype: String::from("tool")};
    let user = Creature {name: String::from("Juho"), symbol: '@', hp: 100, item: item};

    println!("Hello, world! {0}, {1}, {2} with a {3} {4} with {5} uses", user.name, user.symbol, user.hp, user.item.itemtype, user.item.name, user.item.uses);
}


struct Creature {
    name: String,
    symbol: char,
    hp: i32,
    item: Item
}

struct Item {
    name: String,
    itemtype: String,
    uses: i32
}