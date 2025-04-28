fn main() {
    let dice_roll = 5;
    match dice_roll {
        3 => add_fancy_hat(),
        6 => remove_fancy_hat(),
        other => move_player(other),
        //_ => reroll(),  //another way
    }
}

fn add_fancy_hat() {
    println!("You got a Fancy Hat 🧢");
}
fn remove_fancy_hat() {
    println!("Fancy Hat 🧢 removed");
}
fn move_player(num_spaces: u8) {
    println!("Dice Roll = {}. Reroll", num_spaces);
}
