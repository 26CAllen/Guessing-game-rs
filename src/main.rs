fn main() {
    use rand;
    let answer = (rand::random() % 10000) + 1;
    gameLogic(input, answer);
}

fn gameLogic(input: i64, answer: i64) {
    if input==answer {
        // win condition
        println!("Correct!")
    }
    match input > answer {
       True => println!("Too high!"),
       False => println!("Too low!")
    }
}