use dice::throw;


impl Creature {
    pub fn new_random(creature_name: String) -> Creature {
        let item = Item {kind: ItemType::TOOL, name: String::from("lockpick"), uses: -1, damage: String::from("2d4+1")};
        let creature = Creature {name: creature_name, symbol: '@', hp: 5, item: item, initiative: String::from("2d4+0")};
        return creature;
    }

    fn get_damage(&mut self, amount: i32) {
        self.hp -= amount;

        if self.hp < 0 {
            println!("{} is dead!", self.name);
        }
    }
}
pub fn attack(attacker: &mut Creature, defender: &mut Creature) {
    let attacker_first = decide_initiative(&attacker.initiative, &defender.initiative);
    if attacker_first {
    let damage = throw(&attacker.item.damage);
    defender.get_damage(damage);
    } else {
        let damage = throw(&defender.item.damage);
        attacker.get_damage(damage);
    }
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
