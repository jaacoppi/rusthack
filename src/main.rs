fn main() {
    let user = Creature {name: String::from("Juho"), symbol: '@', hp: 100};

    println!("Hello, world! {0}, {1}, {2}", user.name, user.symbol, user.hp);
}


struct Creature {
    name: String,
    symbol: char,
    hp: u32
}