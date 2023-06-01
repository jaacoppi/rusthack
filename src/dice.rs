use diceroll::*;

pub fn throw(damage: &str) -> i32 {
    let v: Vec<&str> = damage.split(|c: char| !c.is_numeric()).collect();

    let dice: i32 = v[0].parse::<i32>().unwrap();
    let sides: i32 = v[1].parse::<i32>().unwrap();
    let modifier: i32 = v[2].parse::<i32>().unwrap();
    let result = roll(DiceRoll::new().dice(dice).sides(sides).modifier(modifier));
    result.total
}
