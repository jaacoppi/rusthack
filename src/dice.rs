use diceroll::*;
extern crate diceroll;

pub fn throw(dice: i32, sides: i32) {
    let modifier = 2;
    let result = roll(DiceRoll::new()
        .dice(dice)
        .sides(sides)
        .modifier(modifier));
    println!("We rolled {}d{}+{}, which yielded a total of {}.", dice, sides, modifier, result.total);
}