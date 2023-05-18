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
