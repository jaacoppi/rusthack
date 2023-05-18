use dice::throw;


pub fn attack(attacker: &mut Creature, defender: &mut Creature) {
    let damage = throw(&attacker.item.damage);
    do_damage(defender, damage);
    println!("{}", defender.hp);
// defender.hp - throw
// status - dead?
}

fn do_damage(target: &mut Creature, amount: i32) {
    target.hp -= amount;

    if target.hp < 0 {
        println!("Dead!");
    }
}

pub fn random_creature() -> Creature {
    let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1, damage: String::from("2d4+1")};
    let creature = Creature {name: String::from("Juho"), symbol: '@', hp: 5, item: item};
    return creature;
}


pub struct Creature {
    pub name: String,
    pub symbol: char,
    pub hp: i32,
    pub item: Item
}

pub struct Item {
    pub kind: ItemType,
    pub name: String,
    pub uses: i32,
    pub damage: String
}

pub enum ItemType {
    TOOL,
    WEAPON
}
