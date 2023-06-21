use crate::dice::throw;
use crate::stats::*;
use rand::*;

impl Creature {
    pub fn new_random() -> Creature {
        let mut rng = thread_rng();
        let hp = rng.gen_range(2..7);
        let item = Item {
            kind: ItemType::TOOL,
            name: String::from("lockpick"),
            uses: -1,
            damage: String::from("2d4+1"),
        };

        let stats = Stats::new();

    match rng.gen_range(1..4) {
	1 => {  return Creature{name: String::from("Tank"), hp: hp+5, item: item, initiative: String::from("1d4+0"), stats} },
    2 => {  return Creature{name: String::from("Fast"), hp: hp, item: item, initiative: String::from("2d4+1"), stats} },
    3 => { return Creature{name: String::from("Default"), hp: hp, item: item, initiative: String::from("1d6+0"), stats }},
    i32::MIN..=0_i32 | 4_i32..=i32::MAX => todo!("Compiler says we need this case. Don't understand why!"),
};


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

    pub fn run(&mut self, enemy: &mut Creature) {
        let attacker_first = decide_initiative(&self.initiative, &enemy.initiative);
        if attacker_first {
            println!("You managed to run away");
            return;
        } else {
            println!("The enemy sees you run and attacks!");
            let damage = throw(&enemy.item.damage);
            self.get_damage(damage);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn format_stats(&self) -> String {
        format!(
            "{0}, {1} hp with a {2} with {3} uses",
            self.name, self.hp, self.item.name, self.item.uses
        )
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
    pub stats: Stats,
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
