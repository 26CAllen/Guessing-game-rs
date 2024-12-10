mod take_input;

fn main() {
    let mut counter: i64 = 0;

    use rand;
    let answer: i64 = (rand::random::<i64>() % 10000) + 1;
    while !game_logic(answer)  {
        counter += 1;
        continue;
    }
    println!("It took you {:?} tries", counter);
}

fn game_logic(answer: i64) -> bool {
    println!("Input a number 1 to 10000: ");
    use take_input::take_input::take_input;
    let input = take_input();
    if input==answer {
        // win condition
        println!("Correct!");
        return true;
    }
    match input > answer {
       true => println!("Too high!"),
       false => println!("Too low!")
    }
    false
}