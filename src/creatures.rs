use dice::throw;


impl Creature {
    pub fn new_random(creature_name: String) -> Creature {
        let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1, damage: String::from("2d4+1")};
        let creature = Creature {name: creature_name, symbol: '@', hp: 5, item: item, initiative: String::from("2d4+0")};
        return creature;
    }


}
pub fn attack(attacker: &mut Creature, defender: &mut Creature) {
    decide_initiative(&attacker.initiative, &defender.initiative);
    let damage = throw(&attacker.item.damage);
    do_damage(defender, damage);
    println!("{}", defender.hp);
// defender.hp - throw
// status - dead?
}

fn decide_initiative(attacker: &String, defender: &String) -> bool {
    let attacker_ini = throw(attacker);
    let defender_ini = throw(defender);

    if attacker_ini >= defender_ini {
        println!("Attacker goes first!");
        return true;
    } else {
        println!("Defender goes first!");
        return false;

    }
}
fn do_damage(target: &mut Creature, amount: i32) {
    target.hp -= amount;

    if target.hp < 0 {
        println!("Dead!");
    }
}

pub struct Creature {
    pub name: String,
    pub symbol: char,
    pub hp: i32,
    pub item: Item,
    pub initiative: String
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
