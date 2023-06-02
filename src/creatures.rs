use crate::dice::throw;
use rand::*;

impl Creature {
    pub fn new_random(creature_name: String) -> Creature {
        let mut rng = thread_rng();
        let hp = rng.gen_range(2..7);
        let item = Item {
            kind: ItemType::TOOL,
            name: String::from("lockpick"),
            uses: -1,
            damage: String::from("2d4+1"),
        };

        Creature {
            name: creature_name,
            hp,
            item,
            initiative: String::from("2d4+0"),
        }
    }

    pub fn attack(&mut self, defender: &mut Creature) {
        let attacker_first = decide_initiative(&self.initiative, &defender.initiative);
        if attacker_first {
            let damage = throw(&self.item.damage);
            defender.get_damage(damage);
        } else {
            let damage = throw(&defender.item.damage);
            self.get_damage(damage);
        }
    }

    fn get_damage(&mut self, amount: i32) {
        println!("{0} receives {1} damage!", self.name, amount);
        self.hp -= amount;

        if !self.is_alive() {
            println!("{} is dead!", self.name);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

fn decide_initiative(attacker: &str, defender: &str) -> bool {
    let attacker_ini = throw(attacker);
    let defender_ini = throw(defender);

    if attacker_ini >= defender_ini {
        println!("Attacker goes first!");
        true
    } else {
        println!("Defender goes first!");
        false
    }
}

pub struct Creature {
    pub name: String,
    pub hp: i32,
    pub item: Item,
    pub initiative: String,
}

pub struct Item {
    pub kind: ItemType,
    pub name: String,
    pub uses: i32,
    pub damage: String,
}

pub enum ItemType {
    TOOL,
    WEAPON,
}
