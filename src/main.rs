mod take_input;

fn main() {
    use take_input::take_input::take_input;
    use rand;
    let answer: i64 = (rand::random::<i64>() % 10000) + 1;
    while game_logic(input, answer) != true {
        continue;
    }
}

fn game_logic(answer: i64) -> bool {
    let input = take_input();
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